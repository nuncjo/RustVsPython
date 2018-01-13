# -*- coding: utf-8 -*-


def hashmaps():

    persons = {}
    persons["Ruben Rybnik"] = 89
    persons.update({
        "Olga Ryba": 55,
        "Jan Rybak": 33,
        "Roman Rybka": 18
    })

    print(f"Check length: {len(persons)}")
    del persons["Roman Rybka"]

    print(f"Check length after remove: {len(persons)}")
    print(f"Contains key Ruben Rybnik: {'Ruben Rybnik' in persons}")
    print(f"Age of Ruben Rybnik is: {persons.get('Ruben Rybnik')}")

    for name, age in persons.items():
        print(f"{name} is {age} years old.")

    for item in persons.keys():
        print(persons[item])

    if "John Nonexistent" not in persons:
        persons["John Nonexistent"] = 100

    print(persons)

    names = dict([
        ("Guido", "VanPossum"),
        ("Kenneth", "Ratz"),
        ("David", "Veazley"),
        ("Mark", "Burtz")
    ])

    print(names)

    # dict comprehension
    squares = {key: key*key for key in range(10)}
    print(squares)

    who = {
        "Jack": "Harkness",
        "Martha": "Jones"
    }
    print(who)


def hashsets():

    chars = set()
    chars.add("A")
    chars.update("A", "B", "C")
    print(chars)

    print(f"Check length: {len(chars)}")

    chars.remove("A")

    print(f"Check length after remove: {len(chars)}")
    print(f"Contains key A: {'A' in chars}")

    for char in chars:
        print(char)

    new_chars = set(["A", "Y", "B", "X"])
    print(new_chars)

    small_chars = {"a", "b", "c"}
    print(small_chars)

    # operations
    print(f"Difference {chars.difference(new_chars)}")
    print(f"Symmetric difference {chars.symmetric_difference(new_chars)}")
    print(f"Intersection {chars.intersection(new_chars)}")
    print(f"Union {chars.union(new_chars)}")


def lists():

    numbers = [1, 2, 3, 4, 5, 6, 7]
    print(numbers)

    print(f"Check length: {len(numbers)}")
    numbers.append(99)
    print(numbers)

    # removes element with index 1
    numbers.remove(1)

    # removes last element
    numbers.pop()

    numbers[0] = 789
    numbers.extend([11, 12, 13])
    print(numbers)
    print(f"Value at index 3 {numbers[3]}")

    for num in numbers:
        print(num)

    zeroes = list(range(0, 5))
    print(zeroes)

    while zeroes:
        print(zeroes.pop())

    new_list = [99, 98, 97]
    # moves values from new_list to numbers, new_list is empty at the end
    while new_list:
        numbers.append(new_list.pop())
    print(numbers)
    print(new_list)

    numbers = numbers[:5]
    print(numbers)

    tmp = [1, 2, 3, 4]
    # //leave only values where match is true
    tmp = [x for x in tmp if x%2 == 0]
    print(tmp)

    # // make vec of numbers in range
    range_list = list(range(20))

    range_list = list(reversed(range_list))
    print(range_list)
    range_list = range_list[::-1]
    print(range_list)
    range_list = [x for x in reversed(range_list)]
    print(range_list)

    # slices are mutable
    slc = range_list[1:4]
    slc.append(1)
    print(slc)

    names = ["Mark", "Zosia", "Kasia", "Marysia"]
    for i, name in enumerate(names):
        print(f"{i} at {name}")

    # list comprehension
    even_squares = [x*x for x in range(0, 10) if x % 2 == 0]
    print(even_squares)


def return_tuple(name):
    if power > 1000:
        return name, power * 3
    return name, power * 2


def tuples():
    thor = ("Thor", true, 345)
    print(thor)
    print(f"{thor[0]}, {thor[1]}, {thor[2]}")

    name, _, power = thor
    print(f"{thor} has {power} points of power", name, power)

    one = 1,
    print(f"single element tuple: {one}");

    god, strength = return_tuple(thor[0], thor[2])
    print(f"This god {god} has now {strength} strength")


if __name__  == "__main__":
    hashmaps()
    hashsets()
    lists()
    tuples()
