##https://www.codewars.com/kata/576bb71bbbcf0951d5000044
def count_positives_sum_negatives(arr):
    min = 0
    max = 0
    res = []
    for x in arr:
        if x > 0:
            max += 1
        if x < 0:
            min += x
    if len(arr) > 0:
        res.append(max)
        res.append(min)
    return res