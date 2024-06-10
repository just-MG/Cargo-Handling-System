import json

def get_input_mode():
    print("<------------------------------>")
    print("Enter input mode: ")
    print("1. Custom input")
    print("2. Select from predefined inputs")
    print("3. Debug mode")
    while True:
        try:
            mode = int(input("Mode: "))
            if mode in [1, 2, 3]:
                return mode
            else:
                print("Invalid input. Please enter 1, 2, or 3.")
        except ValueError:
            print("Invalid input. Please enter a number.")

def get_custom_input():
    print("<------------------------------>")
    print("Color values: 0 - white, 1 - black")
    print("Sample input: 1 1 0 0 0")
    print("Discs are placed in the bins bottom to top")
    user_input = [[] for _ in range(3)]
    for i in range(3):
        while True:
            try:
                print(f"Enter color values for bin {i+1}: ", end="")
                values = list(map(int, input().split()))
                if len(values) != 5 or not all(x in [0, 1] for x in values):
                    print("Invalid input. Enter 5 values of 0 or 1.")
                else:
                    user_input[i] = values
                    break
            except ValueError:
                print("Invalid input. Enter numeric values of 0 or 1.")
    return user_input

def get_debug_input():
    return [[1, 0, 1, 1, 1], [1, 0, 1, 0, 1], [1, 1, 1, 0, 1]]

def get_predefined_input():
    file_path = 'predefined output/predefined_output.JSON'
    key = get_user_char()
    with open(file_path, 'r') as file:
        data = json.load(file)
        if key in data:
            return data[key]
        else:
            print(f"Key '{key}' not found in the JSON file.")
            return None

# Gets a character from the user
# The character should be either a number or a capital letter
def get_user_char():
    while True:
        char = input("Enter a character: ")
        if len(char) == 1 and (char.isdigit() or char.isupper()):
            return char
        else:
            print("Invalid input. Enter a single digit or a capital letter.")
    

def get_input(mode):
    match mode:
        case 1:
            return get_custom_input()
        case 2:
            return get_predefined_input()
        case 3:
            return get_debug_input()

def custom_mapping(char):
    return {0: '.', 1: 'X'}[char]

def print_visualization(user_input):
    print()
    print("Visualization of the input:")
    for i in range(5):
        for j in range(3):
            print(custom_mapping(user_input[j][4-i]), end=" ")
        print()
    print()

class Robot:
    def __init__(self, user_input):
        self.bins = [[] for _ in range(3)]
        self.user_input = user_input
        self.output = [list(user_input[i]) for i in range(3)]

    def next_disk_needed_in_bin(self, bin_no):
        if len(self.bins[bin_no]) == 5:
            return -1
        return self.output[bin_no][len(self.bins[bin_no])]

    def get_sorted_disk_bin(self, disk_color):
        next_bin1 = self.next_disk_needed_in_bin(0)
        next_bin2 = self.next_disk_needed_in_bin(1)
        next_bin3 = self.next_disk_needed_in_bin(2)

        if next_bin1 == disk_color:
            return 0
        elif next_bin2 == disk_color:
            return 1
        elif next_bin3 == disk_color:
            return 2
        else:
            return -1

    def input_disk(self, disk_color, destination):
        self.bins[destination].append(disk_color)

    def visualize(self):
        print()
        print("Visualization of the robot:")
        for i in range(5):
            for j in range(3):
                try:
                    disk = custom_mapping(self.bins[j][4-i])
                except IndexError:
                    disk = " "
                print(disk, end=" ")
            print()
        print()

    def check_finished(self):
        return all(len(bin) == 5 for bin in self.bins)

# Main code
print("Welcome to the software simulation of the robot")

mode = get_input_mode()
user_input = get_input(mode)
robot = Robot(user_input)

while True:
    if robot.check_finished():
        print("Output format achieved. Exiting")
        break
    print("Select action:")
    print("1. Visualize input")
    print("2. Visualize robot")
    print("3. Place disk")
    print("4. Exit")
    print("5. Restart")
    try:
        action = int(input("Action: "))
        if action not in [1, 2, 3, 4, 5]:
            print("Invalid input. Please enter a number between 1 and 5.")
            continue
    except ValueError:
        print("Invalid input. Please enter a numeric value.")
        continue

    match action:
        case 1:
            print_visualization(user_input)
        case 2:
            robot.visualize()
        case 3: 
            try:
                disk_color = int(input("Color (0 - white, 1 - black): "))
                if disk_color not in [0, 1]:
                    print("Invalid color. Please enter 0 or 1.")
                    continue
                disk_color_str = "white" if disk_color == 0 else "black"
                bin_no = robot.get_sorted_disk_bin(disk_color)
                if bin_no == -1:
                    print("Disk cannot be placed in any bin. Discarding.")
                    continue
                print()
                print(f"Next {disk_color_str} disk goes to bin {bin_no+1}")
                robot.input_disk(disk_color, bin_no)
                robot.visualize()
            except ValueError:
                print("Invalid input. Please enter 0 or 1.")
        case 4:
            break
        case 5:
            mode = get_input_mode()
            user_input = get_input(mode)
            robot = Robot(user_input)
