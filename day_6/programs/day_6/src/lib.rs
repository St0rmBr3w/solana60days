use anchor_lang::prelude::*;

declare_id!("4s9fdUYmAGBB8cMKw5KUpedViuRrLNBTyL7KY9euw9cr");

// *** CONSTANT DECLARED HERE ***
const MEANING_OF_LIFE_AND_EXISTENCE: u64 = 42;

#[program]
pub mod day_6 {
    use super::*;
    // Import HashMap library
    use std::collections::HashMap;

    // @dev Mappings in Solana
    pub fn initialize(ctx: Context<Initialize>, key: String, value: String) -> Result<()> {
        // Initialize mapping
        let mut my_map = HashMap::new();

        // Add a key-value pair to the map
        my_map.insert(key.to_string(), value.to_string());

        // Log the value corresponding to a key from the mapping
        msg!("My name is {}", my_map[&key]);

        Ok(())
    }

    // @dev Constants in Solana
    pub fn constants(ctx: Context<Initialize>) -> Result<()> {
        msg!(&format!("Answer to the ultimate question: {}", MEANING_OF_LIFE_AND_EXISTENCE)); // new line here
        Ok(())
    }
    
    // @dev If-Else Statements in Solidity
    // function ageChecker(uint256 age)
    //     public pure returns (string memory) {

    //     if (age >= 18) {
    //         return "You are 18 years old or above";
    //     } else {
    //         return "You are below 18 years old";
    //     }
    // }
    pub fn age_checker_if_else(ctx: Context<Initialize>,
        age: u64) -> Result<()> {
            if age >= 18 {
                msg!("You are 18 years old or above");
            } else {
                msg!("You ae younger than 18 years old");
            }
        Ok(())
    }

    // @dev Ternary Operator in Solidity
    // function ageChecker(uint256 age) public pure returns (bool a) {
	// 	a = age % 2 == 0 ? true : false;
    // }
    pub fn age_checker_ternary(ctx: Context<Initialize>, age: u64) -> Result<()> {

        let result = if age % 2 == 0 {"true"} else {"false"};
        msg!("{:?}", result);
        Ok(())
    }

    pub fn age_checker_match(ctx: Context<Initialize>, age: u64) -> Result<()> {
        match age {
            1 => {
                // Code block executed if age equals 1
                msg!("The age is 1");
            }

            2 | 3 => {
                // Code block executed if age equals 2
                msg!("The age is 2 or 3");
            }

            4..=6 => {
                // Code block executed if age is in the range 4 to 6 (inclusive)
                msg!("The age is between 4 and 6");
            }

            _ => {
                // Code block executed for any other age
                msg!("The age is greater than 6");
            }
        }

        Ok(())
    }

    // function loopOverSmth() public {
    //     for (uint256 i=0; i < 10; i++) {
    //         // do something...
    //     }
    // }
    pub fn for_loops(ctx: Context<Initialize>) -> Result<()> {
        for i in (0..10).step_by(2) {
            // do something...

            // Increment i by 2
            msg!("{}", i);
        }

        Ok(())
    }

    pub fn fixed_array(ctx: Context<Initialize>, num: u32) -> Result<()> {
        // Declare an array of u32 with a fixed size of 5
        let my_array: [u32; 5] = [10, 20, 30, 40, 50];

        // Accessing first array element
        let first_element = my_array[0];
        let third_element = my_array[2];

        // Declare a mutable array of u32 with a fixed size of 3
        let mut mutable_array: [u32; 3] = [100, 200, 300];

        // Change the second element of the array
        mutable_array[1] = num;

        // Rest of your program's logic

        msg!("The second element of your array is {:?}", mutable_array[1]);

        Ok(())
    }

    pub fn dynamic_array(ctx: Context<Initialize>, num: u32) -> Result<()> {
        // Declare a dynamic array-like structure using Vec
        let mut dynamic_array: Vec<u32> = Vec::new();

        // Add your elements to the dynamic array
        dynamic_array.push(10);
        dynamic_array.push(20);
        dynamic_array.push(30);

        // Overwriting the dynamic array
        dynamic_array[2] = num;

        // Accessing elements of the dynamic array
        let first_element = dynamic_array[0];
        let third_element = dynamic_array[2];

        // Rest of your program's logic
        msg!("Third element = {}", third_element);

        Ok(())
    }

    // @dev Struct example in Solidity
    // In the provided code snippet, the Solidity implementation stores 
    // the instance of a struct in storage, whereas in the Solana implementation, 
    // everything happened in the initialize function and nothing was stored on-chain. 
    // Storage will be discussed in a later tutorial.
    // contract SolidityStructs {
    
    //     // Defining a struct in Solidity
    //     struct Person {
    //         string my_name;
    //         uint256 my_age;
    //     }
    
    //     // Creating an instance of the struct
    //     Person person1;
    
    //     function initPerson1(string memory name, uint256 age) public {
    //         // Accessing and modifying struct fields
    //         person1.my_name = name;
    //         person1.my_age = age;
    //     }
    // }
    pub fn structs(ctx: Context<Initialize>, name: String, age: u64) -> Result<()> {
        // Defining a struct in Solana
        struct Person {
            my_name: String,
            my_age: u64,
        }

        // Creating an instance of the struct
        let mut person1: Person = Person {
            my_name: name,
            my_age: age,
        };

        msg!("{} is {} years old", person1.my_name, person1.my_age);

        // Accessing and modifying struct fields
        person1.my_name = "Bob".to_string();
        person1.my_age = 18;

        msg!("{} is {} years old", person1.my_name, person1.my_age);

        Ok(())
    }

    pub fn u_sizing(ctx: Context<Initialize>) -> Result<()> {

        let mut dynamic_array: Vec<u32> = Vec::from([1,2,3,4,5,6]);
        let len = dynamic_array.len(); // this has the usize

        let another_var: u64 = 5; // this has type u64

        let len_plus_another_var = len as u64 + another_var;

        msg!("The result is {}", len_plus_another_var);

        Ok(())
    }

    pub fn even_vector(ctx: Context<Initialize>, vec: Vec<u64>) -> Result<()> {
        let mut even_vec = Vec::new();
        
        for i in 0..vec.len() {
            if vec[i] % 2 == 0 {
                even_vec.push(vec[i]);
            }
        }
        msg!("The even vector is {:?}", even_vec);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
