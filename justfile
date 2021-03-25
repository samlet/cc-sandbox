build:
	python setup.py install

test testname:
    LD_LIBRARY_PATH=/Users/xiaofeiwu/miniconda3/envs/bigdata/lib/ cargo test --no-default-features {{testname}} -- --nocapture
