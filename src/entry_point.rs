pub fn nun_logo() {
    println!(
        r#"
     _   ____  ___   __
    / | / / / / / | / /
   /  |/ / / / /  |/ / 
  / /|  / /_/ / /|  /  
 /_/ |_/\____/_/ |_/

 nun - an operating system framework based on the A9N Microkernel
 
    "#
    );
}

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        $crate::arch_entry!(_entry);

        fn _entry(init_info: *const nun::InitInfo) {
            $crate::entry_point::nun_logo();

            // architecture-independent initialization
            let user_entry: fn(&nun::InitInfo) = $path;

            unsafe {
                user_entry(init_info.as_ref().unwrap());
            }
        }
    };
}
