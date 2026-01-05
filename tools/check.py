#!/usr/bin/env python3
"""Build, format-check and test ODYSSEY without modifying tracked files."""
import os,pathlib,subprocess,sys,tempfile
root=pathlib.Path(__file__).resolve().parents[1]
env=os.environ.copy()
env.setdefault('CARGO_TARGET_DIR',str(pathlib.Path(tempfile.gettempdir())/'odyssey-cargo'))
env.setdefault('GOCACHE',str(pathlib.Path(tempfile.gettempdir())/'odyssey-go-cache'))
env.setdefault('CGO_ENABLED','0')
def run(command,cwd=root):
 subprocess.run(command,cwd=cwd,env=env,check=True)
run(['cargo','fmt','--manifest-path','flight/Cargo.toml','--','--check'])
run(['cargo','test','--manifest-path','flight/Cargo.toml','--quiet'])
run(['go','test','./...'],root/'console')
with tempfile.TemporaryDirectory(prefix='odyssey-erlang-') as temporary:
 sources=sorted((root/'comms/src').rglob('*.erl'))+sorted((root/'comms/test').glob('*.erl'))
 run(['erlc','-Werror','-o',temporary,*map(str,sources)])
 tests=[p.stem for p in (root/'comms/test').glob('*_tests.erl')]
 expression='case eunit:test(['+','.join(tests)+'],[verbose]) of ok -> halt(0); _ -> halt(1) end.'
 run(['erl','+S','2','-noshell','-pa',temporary,'-eval',expression])
print('ODYSSEY checks passed')
