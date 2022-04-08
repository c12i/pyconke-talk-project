def load_file(file_path):
    try:
        with open(file_path) as in_file:
            loaded_txt = in_file.read().strip().split("\n")
            loaded_txt = [x.lower() for x in loaded_txt]
            return loaded_txt
    except IOError as e:
        print(f"{e}\nError opening {file_path}", file=sys.stderr)
        sys.exit(1)