def leap_year(year: int) -> bool:
    # A leap year must be divisible by 4
    if year % 4 != 0:
        return False
    # If divisible by 100, it must also be divisible by 400
    if year % 100 == 0 and year % 400 != 0:
        return False
    return True
