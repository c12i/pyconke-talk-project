import time

import palingrams_pyo3
import utils

if __name__ == '__main__':
	w = utils.load_file('./words.txt')
	start_time = time.time()
	palingrams = palingrams_pyo3.find_palingrams_concurrent(w)
	# palingrams = palingrams_pyo3.find_palingrams(w) 
	end_time = time.time()
	print(f"Runtime for this program was: {end_time - start_time} s")
	print(f"result count = {len(palingrams)}")