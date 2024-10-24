Polecamp is an all-in-one platform for fitness and wellness activities, where instructors from various disciplines—yoga, pole dance, fitness, dance, and more—can create and host classes. It offers seamless class booking, easy payment options, and a robust rewards system for both instructors and participants. The platform also supports collaboration between multiple instructors, enabling them to co-host events or workshops, creating a richer experience for users.

Functions:

1. createEvent - payable (only instructors)
2. joinEvent - payable (users)
3. endEvent - (event creators)
4. createGroupEvent - payable (only instructors) 
    -- add addresses of instructors
5. createInstructorProfile
6. approveInstructorProfile (owner of contract / polecamp)
7. rateInstructor -- (instructorId, eventId)
8. mintNFT -- event completion for users, instructors
9. addPoints -- on event completion

Structs:

struct User {
    ipfs_url: string,
    address: address
    category: string
    type: string
}

Variables:

pendingInstructors -- Vec<Instrutor>
Instructors -- Vec<Instrutor>
events - mapping<address - eventid>
event_instructors -- mapping<eventid - Vec<address>
event_users -- mapping<eventid - Vec<address>



Events:

InstructorCreated
InstructorApproved
EventCreated


