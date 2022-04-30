import time
from palingrams_cpython import find_palingrams_concurrent

import utils

if __name__ == '__main__':
	w = utils.load_file('./words.txt')
	start_time = time.time()
	palingrams = find_palingrams_concurrent(w)
	end_time = time.time()
	print(f"Runtime for this program was: {end_time - start_time} s")
	print(f"result count = {len(palingrams)}")