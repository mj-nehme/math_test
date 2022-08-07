# math_test

A math test prepared for my kids who are still in their elementary school.

🚧

⚠️  **This project is under construction** 

## Description
The student can choose one exam out of six:
- Addition
- Subtraction
- Multiplication
- Division
- One Variable Equation
- Two Variables Equation (unimplemented yet)

The student has the option to choose the complexity level up to level 10.

**An exam is made up of 10 questions, where the student should answer each within 10 secs. After the timeout, the answer is considered to be wrong.**

At the end, the score is calculated out of 10.

## Next steps:
- Add documentation
- Make it accessible on other platforms that don't have the same CPU architecture. Either:
    - Transform to Wasm (Handle the needed changes to be convertible). And then embed it into some JS/HTML.
    - Add a RESTful API (most likely Actix).
- Perform the minor TODOs (mentioned in the code).

## NOTE
The [math_test crate](https://crates.io/crates/math_test) would be frequently updated on major changes, but the [math_test github repo](https://github.com/mj-nehme/math_test) is to be updated on every (even minor) change.

