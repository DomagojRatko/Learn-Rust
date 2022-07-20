/**
 * In this module we will go through few examples and explain Vector in rust lang.
 * What is Vector explained. (line 15)
 * Vector example and syntax. (line 27)
 * Vector new(); example. (line 31)
 * Vector vec! Macro example. (line 39)
 * Vector push(); example. (line 43)
 * Vector remove(); example. (line 50)
 * Vector contains(); example. (line 55)
 * Vector len(); example. (line 62)
 * Accessing values from a vector. (line 66)
 */
pub mod module {
    pub fn vectors() {
        // A Vector is a resizable array. It stores values in contiguous memory blocks.
        // The predefined structure Vec can be used to create vectors.
        // Some important features of a Vector are.
        //
        // A Vector can grow or shrink at runtime.
        // A Vector is a homogeneous collection.
        // A Vector stores data as sequence of elements in a particular order. Every element in a Vector is assigned a unique index number.
        // The index starts from 0 and goes up to n-1 where, n is the size of the collection.
        // For example, in a collection of 5 elements, the first element will be at index 0 and the last element will be at index 4.
        // A Vector will only append values to (or near) the end. In other words, a Vector can be used to implement a stack.
        // Memory for a Vector is allocated in the heap.

        // Vector example and syntax.
        // let mut instance_name = Vec::new();
        // let vector_name = vec![val1,val2,val3];

        // Vector new();
        let mut v = Vec::new();
        v.push(20);
        v.push(30);
        v.push(40);
        println!("size of vector is :{}",v.len());
        println!("{:?}",v);

        // vec! Macro.
        let v1 = vec![1,2,3];
        println!("{:?}",v1);

        // Vector push();
        let mut v2 = Vec::new();
        v2.push(20);
        v2.push(30);
        v2.push(40);
        println!("{:?}",v2);

        // Vector remove();
        let mut v3 = vec![10,20,30];
        v3.remove(1);
        println!("{:?}",v3);

        // Vector contains();
        let v4 = vec![10,20,30];
        if v4.contains(&10) {
            println!("found 10");
        }
        println!("{:?}",v4);

        // Vector len();
        let v5 = vec![1,2,3];
        println!("size of vector is :{}",v5.len());

        // Accessing values from a vector.
        let mut v6 = Vec::new();
        v6.push(20);
        v6.push(30);
        println!("{:?}",v6[0]);
        // or using reference to the collection.
        let mut v7 = Vec::new();
        v7.push(20);
        v7.push(30);
        v7.push(40);
        v7.push(500);
        for i in &v7 {
            println!("{}",i);
        }
        println!("{:?}",v7);
    }
}