#[derive(Debug)]

enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

#[derive(Debug, Copy,Clone)]
struct UserId(u64);

#[derive(Debug,Copy,Clone)]
struct TopicId(u64);

#[derive(Debug)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug)]
struct Topic {
    id: TopicId,
    name: String,
    owner: UserId,
}

#[derive(Debug)]
enum Event {
    Join((UserId,TopicId)),
    Leave((UserId,TopicId)),
    Message((UserId,TopicId,String)),
}


fn porcess_event(event: &Event) {

    match event {
        Event::Join((user_id, topic_id)) => {
            println!("{:?} join topic {:?}", user_id, topic_id);
        },
        Event::Leave((user_id, topic_id)) => {
            println!("{:?} leave topic {:?}", user_id, topic_id);
        },
        Event::Message((user_id, topic_id, message)) => {
            println!("{:?} send message {:?} to topic {:?}", user_id, message, topic_id);
        }
    }
}


fn process_message( event: &Event) {
 
    if let Event::Message((user_id, topic_id, message)) = event {
        println!("{:?} send message {:?} to topic {:?}", user_id, message, topic_id);
    }
    
}

fn main() {
    
    let user1 = User {
                        id: UserId(1),
                        name: "Alice".to_string(),
                        gender: Gender::Female
                    };

    let user2 = User {
                        id: UserId(2),
                        name: "Bob".to_string(),
                        gender: Gender::Male
                    };

    let topic = Topic { id: TopicId(1), name: "new topic".to_string(), owner: UserId(1)};

    let event1 = Event::Join((user1.id, topic.id));
    let event2 = Event::Join((user2.id,topic.id));
    let event3 = Event::Message((user1.id,topic.id,"hello".to_string()));
    
                
    println!("event1:{:?},event2:{:?},event3:{:?}",event1,event2,event3);

}




