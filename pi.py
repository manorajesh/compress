from decimal import *

def pi_digit_at_position(position):
    # Set the precision of the decimal module
    getcontext().prec = position + 1
    
    # Calculate pi using the BBP formula
    pi = Decimal(0)
    for k in range(position + 1):
        pi += (Decimal(4) / (Decimal(8) * Decimal(k) + Decimal(1)) - Decimal(2) / (Decimal(8) * Decimal(k) + Decimal(4)) - Decimal(1) / (Decimal(8) * Decimal(k) + Decimal(5)) - Decimal(1) / (Decimal(8) * Decimal(k) + Decimal(6))) * Decimal(16) ** Decimal(position - k)
    
    # Return the digit at the specified position
    pi_str = str(pi)
    return int(pi_str[position])


# Generate the digit at position 100
digit = pi_digit_at_position(30000)
print(digit)
