#!/usr/bin/env python3
"""Focused real helper acceptance using disposable Codex session fixtures only."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import shutil
import sqlite3
import tempfile

STAMP = "2026-01-01T00:00:00.000Z"
IDS = [f"00000000-0000-4000-8000-{n:012d}" for n in range(1, 8)]


def meta(thread, mode="legacy", **extra):
    return {"id":thread,"timestamp":STAMP,"cwd":"/fixture/project","originator":"fixture","cli_version":"0.167.2","source":"cli","history_mode":mode, **extra}


def event(kind, **payload):
    return ("event_msg", {"type":kind, **payload})


def turn(thread, turn_id, question, answer, modern=False):
    records = [event("task_started", turn_id=turn_id)]
    if modern:
        for item in [
            {"type":"UserMessage","id":turn_id+"-user","content":[{"type":"text","text":question}]},
            {"type":"AgentMessage","id":turn_id+"-agent","content":[{"type":"Text","text":answer}]},
        ]:
            records.append(event("item_completed", thread_id=thread, turn_id=turn_id, item=item, completed_at_ms=1))
    else:
        records += [event("user_message",message=question),event("agent_message",message=answer)]
    records.append(event("task_complete", turn_id=turn_id))
    return records


def write(root, thread, metadata, records, start=0):
    path = root / "sessions/2026/01/01" / f"rollout-2026-01-01T00-00-00-{thread}.jsonl"
    path.parent.mkdir(parents=True,exist_ok=True)
    values = [("session_meta",metadata)] + records
    path.write_text("".join(json.dumps({"timestamp":STAMP,"ordinal":i,"type":kind,"payload":payload})+"\n" for i,(kind,payload) in enumerate(values,start)))
    return path


def invoke(executable, root, op, **fields):
    with tempfile.TemporaryDirectory(prefix="fetch-codex-request-") as work:
        request={"op":op,"source_roots":[str(root/"sessions"),str(root/"archived_sessions")],"work_dir":work,**fields}
        process=subprocess.run([executable],input=json.dumps(request),text=True,capture_output=True,timeout=120)
        result=json.loads(process.stdout)
        return process,result


def main():
    parser=argparse.ArgumentParser()
    parser.add_argument("executable")
    args=parser.parse_args()
    with tempfile.TemporaryDirectory(prefix="fetch-codex-original-fixtures-") as directory:
        root=Path(directory)
        (root/"archived_sessions").mkdir()
        long_answer="Full assistant reply.\n"*10000
        injection=("response_item",{"type":"message","role":"user","content":[{"type":"input_text","text":"<recommended_plugins>injected</recommended_plugins>"}]})
        legacy=write(root,IDS[0],meta(IDS[0]),[injection]+turn(IDS[0],"legacy-1","typed first",long_answer)+turn(IDS[0],"legacy-2","typed second","final second"))
        records=[]
        for n in range(140): records += turn(IDS[1],f"modern-{n}",f"typed {n}",f"answer {n}",True)
        native_tool_output=[{"type":"input_text","text":"exact native tool output"}]
        records.insert(-1,event("item_completed",thread_id=IDS[1],turn_id="modern-139",item={"type":"FunctionCallOutput","id":"tool-call-1","name":"fixture_tool","output":native_tool_output},completed_at_ms=2))
        modern=write(root,IDS[1],meta(IDS[1],"paginated"),records)
        parent=write(root,IDS[2],meta(IDS[2],"paginated"),turn(IDS[2],"parent-1","before fork","inherited reply",True)+turn(IDS[2],"parent-2","after cutoff","must stay hidden",True))
        child=write(root,IDS[3],meta(IDS[3],"paginated",history_base={"thread_id":IDS[2],"end_ordinal_exclusive":5,"end_byte_offset":sum(len(line) for line in parent.read_bytes().splitlines(keepends=True)[:5])}),turn(IDS[3],"child-1","fork question","fork reply",True),start=5)
        subagent=write(root,IDS[4],meta(IDS[4],"paginated",source={"subagent":"review"},subagent_history_start_ordinal=1),turn(IDS[4],"child-agent","agent task","agent reply",True))
        original={p:hashlib.sha256(p.read_bytes()).hexdigest() for p in root.rglob("*.jsonl")}
        proc,result=invoke(args.executable,root,"conversation",file_path=str(legacy))
        assert proc.returncode==0,result
        assert [(m["kind"],m.get("text")) for m in result["messages"]]==[("user","typed first"),("assistant",long_answer),("user","typed second"),("assistant","final second")],result
        print("PASS: legacy full manual messages and full assistant text, injected context excluded")
        proc,result=invoke(args.executable,root,"conversation",file_path=str(modern))
        assert proc.returncode==0,result
        assert len(result["messages"])==281,result
        assert result["messages"][-2]["text"]=="answer 139",result
        assert result["messages"][-1]["tool_name"]=="fixture_tool",result
        assert result["messages"][-1]["output"]==native_tool_output,result
        print("PASS: modern native pagination reads all 281 messages and exact named tool result, including beyond first page")
        proc,result=invoke(args.executable,root,"conversation",file_path=str(child))
        assert proc.returncode==0,result
        assert [m["text"] for m in result["messages"]]==["before fork","inherited reply","fork question","fork reply"],result
        print("PASS: native lineage includes inherited turns and respects fork cutoff")
        proc,result=invoke(args.executable,root,"list",candidates=[{"id":str(i),"file_path":str(path)} for i,path in enumerate([legacy,modern,parent,child,subagent])])
        assert proc.returncode==0,result
        assert len(result["sessions"])==4,result
        proc,result=invoke(args.executable,root,"conversation",file_path=str(subagent))
        assert proc.returncode!=0 and "error" in result,result
        print("PASS: native subagents excluded from main list and direct main conversation read")
        # Build a disposable native index from a private projection, never personal state.
        with tempfile.TemporaryDirectory(prefix="fetch-codex-index-fixture-") as work:
            request={"op":"conversation","source_roots":[str(root/"sessions"),str(root/"archived_sessions")],"work_dir":work,"file_path":str(legacy)}
            process=subprocess.run([args.executable],input=json.dumps(request),text=True,capture_output=True,timeout=120)
            assert process.returncode==0,process.stdout
            index=root/"state_5.sqlite"
            fixture_source=Path(work)/"codex-snapshot/state_5.sqlite"
            with sqlite3.connect(f"file:{fixture_source}?mode=ro",uri=True) as native_database, sqlite3.connect(index) as destination:
                native_database.backup(destination)
        duplicate=legacy.with_name(legacy.stem+"_"+IDS[5]+".jsonl")
        shutil.copy2(legacy,duplicate)
        with sqlite3.connect(index) as database:
            database.execute("UPDATE threads SET rollout_path = ?, title = ? WHERE id = ?",(str(legacy.resolve()),"Native saved conversation name",IDS[0]))
        before_index=hashlib.sha256(index.read_bytes()).hexdigest()
        proc,result=invoke(args.executable,root,"list",source_index=str(index),candidates=[{"id":"selected","file_path":str(legacy)},{"id":"stale","file_path":str(duplicate)}])
        assert proc.returncode==0,result
        assert [item["id"] for item in result["sessions"]]==["selected"],result
        assert result["sessions"][0]["native_selection_source"]=="state_index",result
        assert result["sessions"][0]["name"]=="Native saved conversation name",result
        assert hashlib.sha256(index.read_bytes()).hexdigest()==before_index
        # A removed current rollout must not suppress other conversations or
        # resurrect the surviving older candidate for that same thread.
        with sqlite3.connect(index) as database:
            database.execute("UPDATE threads SET rollout_path = ? WHERE id = ?",(str(legacy.with_name("missing-current-rollout.jsonl")),IDS[0]))
        before_missing_index=hashlib.sha256(index.read_bytes()).hexdigest()
        before_missing_sources={p:hashlib.sha256(p.read_bytes()).hexdigest() for p in [*original,duplicate]}
        proc,result=invoke(args.executable,root,"list",source_index=str(index),candidates=[{"id":"older-surviving","file_path":str(legacy)},{"id":"valid-other","file_path":str(modern)}])
        assert proc.returncode==0,result
        assert [item["id"] for item in result["sessions"]]==["valid-other"],result
        assert hashlib.sha256(index.read_bytes()).hexdigest()==before_missing_index
        assert {p:hashlib.sha256(p.read_bytes()).hexdigest() for p in before_missing_sources}==before_missing_sources
        print("PASS: missing indexed current rollout skips only that thread, never falls back, and preserves index/session bytes")
        index.unlink()
        duplicate.unlink()
        print("PASS: native read-only index selects current rollout and leaves index bytes unchanged")
        assert {p:hashlib.sha256(p.read_bytes()).hexdigest() for p in original}==original
        assert not list(root.rglob("*.sqlite"))
        print("PASS: all original session hashes unchanged; no source-side databases created")
        parent.unlink()
        proc,result=invoke(args.executable,root,"conversation",file_path=str(child))
        assert proc.returncode!=0 and result["error"]["code"]=="CODEX_HISTORY_ANCESTOR_MISSING",result
        assert result.get("complete") is not True,result
        print("PASS: missing native ancestry returns an explicit error, never partial success")

if __name__=="__main__": main()
