"""Compare actual native captures with Figma at equal pixel size.

The images here are verification evidence. This script does not alter app assets.
Metrics describe remaining error; they do not define an invented pass threshold.
"""
from pathlib import Path
import json
from PIL import Image, ImageChops, ImageEnhance

ROOT=Path(__file__).resolve().parent

def convert_raw():
    for path in ROOT.glob('*.rgba'):
        width,height=map(int,path.stem.rsplit('-',1)[1].split('x'))
        Image.frombytes('RGBA',(width,height),path.read_bytes()).save(path.with_suffix('.png'))
        path.unlink()

def measure(reference,native,region=None,exclude_text=False,ignore_rects=()):
    origin=region[:2] if region else (0,0)
    if region:
        reference=reference.crop(region);native=native.crop(region)
    errors=[];same=0;count=0
    for index,(a,b) in enumerate(zip(reference.getdata(),native.getdata())):
        x=origin[0]+index%reference.width;y=origin[1]+index//reference.width
        if any(left<=x<right and top<=y<bottom for left,top,right,bottom in ignore_rects): continue
        if exclude_text and (max(a[:3])<130 or max(b[:3])<130):
            continue
        e=[abs(x-y) for x,y in zip(a[:3],b[:3])]
        errors.extend(e);same+=not any(e);count+=1
    return {'pixels':count,'mean_absolute_RGB_error_0_to_255':round(sum(errors)/len(errors),4),'max_channel_error':max(errors),'exact_RGB_match_percent':round(100*same/count,3)}

def save_pair(reference,native,name):
    assert reference.size==native.size
    pair=Image.new('RGB',(reference.width*2,reference.height),(245,245,245))
    pair.paste(reference,(0,0));pair.paste(native,(reference.width,0))
    pair.save(ROOT/f'{name}-reference-left-native-right.png')
    ImageEnhance.Brightness(ImageChops.difference(reference.convert('RGB'),native.convert('RGB'))).enhance(4).save(ROOT/f'{name}-difference-4x.png')

convert_raw()
report={'explanation':'RGB error at identical dimensions. No threshold converts these measurements into a pixel-perfect claim. Component states use the official Figma bounds; app reference removes only40pxpresentationmargin and32pxOSbar.'}
reference=Image.open(ROOT/'figma-buttons.png').convert('RGBA')
native=Image.open(ROOT/'native-buttons-524x208.png').convert('RGBA')
save_pair(reference,native,'buttons')
report['buttons_full']=measure(reference,native)
report['buttons_excluding_dark_text_pixels']=measure(reference,native,exclude_text=True)
report['buttons_decoration_excluding_text_boxes']=measure(reference,native,ignore_rects=[(48,42,93,62),(189,42,234,62),(330,42,375,62),(48,146,95,166),(431,10,476,30)])
for name,(x,y,w,h) in {'enabled':(32,32,77,40),'disabled':(173,32,77,40),'hovered':(314,32,77,40),'blue_action':(32,136,79,40),'selected':(415,0,77,40)}.items():
    report[name]={'face':measure(reference,native,(x,y,x+w,y+h)), 'reference_quiet_point':reference.getpixel((x+w//2,y+10)),'native_quiet_point':native.getpixel((x+w//2,y+10))}
if (ROOT/'native-design-tools-1248x1128.png').exists():
    reference=Image.open(ROOT/'figma-app.png').convert('RGBA').crop((40,72,1288,1200))
    native=Image.open(ROOT/'native-design-tools-1248x1128.png').convert('RGBA')
    save_pair(reference,native,'tools-app')
    report['tools_app_full']=measure(reference,native)
    report['tools_app_excluding_dark_text_pixels']=measure(reference,native,exclude_text=True)
cold=ROOT/'native-design-tools-1248x1128.png';warm=ROOT/'native-design-tools-cached-1248x1128.png'
if cold.exists() and warm.exists():
    report['cold_and_cached_tools_pixels_exactly_equal']=Image.open(cold).convert('RGBA').tobytes()==Image.open(warm).convert('RGBA').tobytes()
(ROOT/'pixel-comparison.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report,indent=2))
