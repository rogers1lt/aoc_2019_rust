//Copyright Andrew Rogers 2019
//Advent of Code 2019 Day 1a

fn calc_fuel(mass: i64) -> i64 {
    unimplemented!();
}

#[test]
fn test_calc_fuel() {
    asserteq!(calc_fuel(12), 2);
    asserteq!(calc_fuel(14), 2);
    asserteq!(calc_fuel(1969), 654);
    asserteq!(calc_fuel(100756), 33583);
    
// For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to get 2.
// For a mass of 14, dividing by 3 and rounding down still yields 4, so the fuel required is also 2.
// For a mass of 1969, the fuel required is 654.
// For a mass of 100756, the fuel required is 33583.
}