f = open("input", "r")
d = f.read()


root_folder = {
    "folders": {},
    "files": [],
    "parent": None,
}
current_folder = root_folder

for line in d.splitlines()[1:]:
    if line.startswith("$ cd "):
        folder_name = line[5:]

        if folder_name == "..":
            current_folder = current_folder["parent"]
        else:
            current_folder = current_folder["folders"][folder_name]
    elif line.startswith("$ ls"):
        pass
    else:
        size_or_dir, name = line.split(" ")
        if size_or_dir == "dir":
            sub_folder = {
                "folders": {},
                "files": [],
                "parent": current_folder
            }
            current_folder["folders"][name] = sub_folder
        else:
            file = {
                "name": name,
                "size": int(size_or_dir)
            }
            current_folder["files"].append(file)

def calc_folder_size(current_folder):
    sub_folders_size = 0
    for folder in current_folder["folders"]:
        sub_folders_size += calc_folder_size(current_folder["folders"][folder])

    files_size = 0
    for file in current_folder["files"]:
        files_size += file["size"]

    current_folder["size"] = files_size + sub_folders_size
    return files_size + sub_folders_size

calc_folder_size(root_folder)

def sum_folder_size(current_folder):
    size = 0
    for folder in current_folder["folders"]:
        size += sum_folder_size(current_folder["folders"][folder])

    if current_folder["size"] <= 100000:
        size += current_folder["size"]

    return size

print(sum_folder_size(root_folder))

space_available = 70000000 - root_folder["size"]
space_needed = 30000000 - space_available

def collect_folder_size(current_folder, size_list):
    for folder in current_folder["folders"]:
        collect_folder_size(current_folder["folders"][folder], size_list)

    size_list.append(current_folder["size"])

size_list = []
collect_folder_size(root_folder, size_list)
size_list.sort()
for size in size_list:
    if size >= space_needed:
        print(size)
        break
