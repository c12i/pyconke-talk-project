import sys
import time

def load_file(file_path):
    try:
        with open(file_path) as in_file:
            loaded_txt = in_file.read().strip().split("\n")
            loaded_txt = [x.lower() for x in loaded_txt]
            return loaded_txt
    except IOError as e:
        print(f"{e}\nError opening {file_path}", file=sys.stderr)
        sys.exit(1)

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
    w = load_file("./words.txt")
    start_time = time.time()
    palingrams = find_palingrams(w)
    end_time = time.time()
    print(palingrams)
    print(f"Runtime for this program was: {end_time - start_time} s")
    print(f"result count = {len(palingrams)}")
