# rust_setup
This will setup rust projects fast

# Design Patterns
- DRY:
    Dont repeat yourself
- KISS:
    Keep it simple, stupid!
    Dont make it impossible to use, by making the type names weird. 
    Make it understandable for a stupid baby!
- State:
    ```rust
    //Example struct
    pub struct Example<State> {
        state: std::marker::PhantomData<State>
    }

    //Impliment functions for all states
    impl<State> Example<State>{
        pub fn all_states(){

        }
    }
    
    pub struct State1;
    //Impliment functions for all states
    impl Example<State1>{
        pub fn only_state1(){
            
        }
    }
    
    pub struct State2;
    //Impliment functions for all states
    impl Example<State2>{
        pub fn only_state2(){
            
        }
    }
    ```
- Builder:
    Builder pattern, for easy initialization and definition!
- Factory
    Every shared fn or type should be in a trait!
