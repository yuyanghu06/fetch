# Setup reads

The existing discovery GET validates one optional absolute path, then calls the read-only discovery service. Management permission is checked first. It never installs packages, starts helpers, logs in or migrates credentials. Installation operations are not implemented in this increment.
