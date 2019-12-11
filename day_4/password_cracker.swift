/// Function to determine the validity for a password.
func is_valid_password(pw: Int) -> Bool {
    let s = String(pw)

    // check, if password is monotonically increasing
    let (increases_monotonically, _) = s.reduce((true, "0"), { (acc: (Bool, Character), cur: Character) -> (Bool, Character) in
        return (acc.0 && acc.1 <= cur, cur)
    })

    // check, if one number occurs twice in a row
    var seen_pair = false
    var is_valid = false
    var has_adjacent_repeating_numbers = false
    var cur: Character = "0"
    for c in s {
        if c == cur {
            if seen_pair {
                // this makes this invalid
                is_valid = false
            } else {
                seen_pair = true
                is_valid = true
            }
        } else {
            if seen_pair && is_valid {
                has_adjacent_repeating_numbers = true
            }
            seen_pair = false
            is_valid = false
        }
        cur = c
    }
    // for the final element of the string
    if seen_pair && is_valid {
        has_adjacent_repeating_numbers = true
    }

    return increases_monotonically && has_adjacent_repeating_numbers
}

let lower = 264360
let upper = 746325

var valid_passwords = Array<Int>()
for num in Range(uncheckedBounds: (lower, upper)) {
    // convert number to string
    if is_valid_password(pw: num) {
        valid_passwords.append(num)
    }
}

print("Number of valid passwords within range: \(valid_passwords.count)")

