class Person:
    def __init__(self, age, gender) -> None:
        self.age = age
        self.gender = gender

john = Person(25, "male")
print(john)

# What you see:
str = "Hello World"

# # What your computer sees:
str = "Hello World" 
str_address = 0x1f94753b8b0 #Don't let this crazy string of numbers and letters spook you!
# #The computer recognizes the line above as an address in the physical memory of your computer
# #now that address has a value "Hello World", and can be used elsewhere

# print(str) #let's say we use our variable x for the last time here
# #at this point, the garbage collector sweeps through and cleans up the memory we allocated to str
# #str no longer exists in the physical memory of your computer