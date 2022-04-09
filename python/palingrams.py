import time

import utils

def find_palingrams(word_list):
    palingram_list = []
    for word in word_list:
        word_len = len(word)
        rev_word = word[::-1]
        
        if word_len > 1:
            for idx in range(word_len):
                if word[idx:] == rev_word[:word_len-idx] and rev_word[word_len-idx:] in word_list:
                    palingram_list.append((word, rev_word[word_len-idx:]))
                if word[:idx] == rev_word[word_len-idx:] and rev_word[:word_len-idx] in word_list:
                    palingram_list.append((word, rev_word[:word_len-idx]))

    return palingram_list

if __name__ == "__main__":
    w = utils.load_file("./words.txt")
    start_time = time.time()
    palingrams = find_palingrams(w)
    end_time = time.time()
    print(palingrams)
    print(f"Runtime for this program was: {end_time - start_time} s")
    print(f"result count = {len(palingrams)}")
