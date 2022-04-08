from itertools import chain
from math import ceil
import os
import time
import concurrent.futures

import utils

source_word_list = utils.load_file('./words.txt')

def find_palingrams(word_list_chunk):
	palingram_list = []
	for word in word_list_chunk:
			word_len = len(word)
			rev_word = word[::-1]
			if word_len > 1:
					for idx in range(word_len):
							if word[idx:] == rev_word[:word_len-idx] and rev_word[word_len-idx:] in source_word_list:
									palingram_list.append((word, rev_word[word_len-idx:]))
							if word[:idx] == rev_word[word_len-idx:] and rev_word[:word_len-idx] in source_word_list:
									palingram_list.append((word, rev_word[:word_len-idx]))
	return palingram_list

if __name__ == '__main__':
	chunk_size = ceil(len(source_word_list) / os.cpu_count()) 
	chunks = list(utils.split_list_to_chunks(source_word_list, chunk_size))
	start_time = time.time()
	palingram_list = None
	with concurrent.futures.ThreadPoolExecutor(max_workers=os.cpu_count()) as executor:
		futures = [executor.submit(find_palingrams, c) for c in chunks]
		palingram_list = list(chain.from_iterable([f.result() for f in futures]))
	print(palingram_list)
	print(f"Runtime for this program was: {time.time() - start_time} s")
	print(f"result count = {len(palingram_list)}")