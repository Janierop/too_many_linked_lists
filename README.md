This repo shows my efforts at following the rust guide at https://rust-unofficial.github.io/too-many-lists 
where you learn most of the features that rust has to offer by implementing too many linked lists. This guide is one
of the most popular guides for new rust developers.

I even spotted an edge case that the original author, who is one of the original authors of the rust collections library, seems to have overlooked in their most complicated "production ready" implementation of the LinkedList.
I checked source code of the rust collections library to see if the mistake is also there and fortunately it is was not. I have also made an additional test called test_cursor_split_ends() that tests for this edge case. 
[sixth_fault.rs](src/sixth_faulty.rs#L1206) contains the author's original faulty code found in their guide and [sixth.rs](src/sixth.rs#L1113) contains my corrected version and both files now contain the test for the overlooked edge case. 