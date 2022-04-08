import time

import palingrams_cpython
import utils

if __name__ == '__main__':
	w = utils.load_file('./words.txt')
	start_time = time.time()
	palingrams = palingrams_cpython.find_palingrams_concurrent(w)
	# palingrams = palingrams_cpython.find_palingrams(w) <-- uncomment this and comment out above code to try out single threaded variant
	end_time = time.time()
	print(palingrams)
	print(f"Runtime for this program was: {end_time - start_time} s")
	print(f"result count = {len(palingrams)}")