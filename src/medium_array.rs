
            pub fn index_array_repeat_stack_32(x: usize)-> u8 {
                let arr: [u8; 32] = [42; 32];
                arr[x]
            }

            pub fn index_array_repeat_rodata_32(x: usize)-> u8 {
                let arr: [u8; 32] = *&[42; 32];
                arr[x]
            }
        

            pub fn index_array_repeat_stack_64(x: usize)-> u8 {
                let arr: [u8; 64] = [42; 64];
                arr[x]
            }

            pub fn index_array_repeat_rodata_64(x: usize)-> u8 {
                let arr: [u8; 64] = *&[42; 64];
                arr[x]
            }
        

            pub fn index_array_repeat_stack_128(x: usize)-> u8 {
                let arr: [u8; 128] = [42; 128];
                arr[x]
            }

            pub fn index_array_repeat_rodata_128(x: usize)-> u8 {
                let arr: [u8; 128] = *&[42; 128];
                arr[x]
            }
        

            pub fn index_array_repeat_stack_256(x: usize)-> u8 {
                let arr: [u8; 256] = [42; 256];
                arr[x]
            }

            pub fn index_array_repeat_rodata_256(x: usize)-> u8 {
                let arr: [u8; 256] = *&[42; 256];
                arr[x]
            }
        
