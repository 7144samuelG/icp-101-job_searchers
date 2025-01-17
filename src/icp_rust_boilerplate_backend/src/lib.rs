#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct User{
    username:String,
    id:u64,
    useremail:String,
    phonenumber:String,
    created_at:u64,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Job{
    id:u64,
    by:String,
    companyname:String,
    companyemail:String,
    companyphonenumber:String,
    position:String,
    jobname:String,
    salary:String,
    requirements:String,
    applicationdeadeline:String,
    created_at:u64,
    
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct EnquireAboutAJob{
    id:u64,
    by:String,
    jobname:String,
    question:String,
    usernamemail:String,
    created_at:u64
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Comment{
    id:u64,
    by:String,
    comment:String,
    useremail:String,
    created_at:u64
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Question{
    id:u64,
    question:String,
    useremail:String,
    created_at:u64
}

impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for User {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Job {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Job {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for EnquireAboutAJob {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for EnquireAboutAJob {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Comment {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Comment {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Question {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Question {
    const MAX_SIZE: u32 = 512;
    const IS_FIXED_SIZE: bool = false;
}

// thread
thread_local! {
    static MEMORY_MANAGER:RefCell<MemoryManager<DefaultMemoryImpl>>=RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );
    static ID_COUNTER:RefCell<IdCell>=RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))),0).expect("Cannot create a counter")
    );
    static USERS_STORAGE:RefCell<StableBTreeMap<u64,User,Memory>>=RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
    static JOBS_STORAGE:RefCell<StableBTreeMap<u64,Job,Memory>>=RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));
    static QUESTION_STORAGE:RefCell<StableBTreeMap<u64,EnquireAboutAJob,Memory>>=RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3)))
    ));
    static COMMENTS_STORAGE:RefCell<StableBTreeMap<u64,Comment,Memory>>=RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4)))
    ));
    static QUESTIONS_STORAGE:RefCell<StableBTreeMap<u64,Question,Memory>>=RefCell::new(StableBTreeMap::init(
        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5)))
    ));
}


#[derive(candid::CandidType,Clone,Serialize,Deserialize,Default)]
struct UserPayload{
    username:String,
    phonenumber:String,
    useremail:String,   
}

#[derive(candid::CandidType,Serialize,Deserialize,Default)]

struct JobPayload{
    by:String,
    companyname:String,
    companyemail:String,
    companyphonenumber:String,
    position:String,
    jobname:String,
    salary:String,
    requirements:String,
    applicationdeadeline:String,
}

#[derive(candid::CandidType,Serialize,Deserialize,Default)]
struct EnquirePayload{
    by:String,
    jobname:String,
    question:String,
    usernamemail:String,

 }
#[derive(candid::CandidType,Serialize,Deserialize,Default)]
struct CommentPayload{
    by:String,
    comment:String,
    useremail:String,
}
#[derive(candid::CandidType,Clone,Serialize,Deserialize,Default)]

struct QuestionPayload{
    question:String,
    useremail:String,
}

#[derive(candid::CandidType,Serialize,Deserialize,Default)]
struct DeleteJobPayload{
    jobid:u64,
    username:String
}
#[derive(candid::CandidType,Serialize,Deserialize,Default)]
struct UpdateJobsPayload{
    jobid:u64,
    by:String,
    companyname:String,
    companyemail:String,
    companyphonenumber:String,
    position:String,
    jobname:String,
    salary:String,
    requirements:String,
    applicationdeadeline:String,
}

#[derive(candid::CandidType,Serialize,Deserialize,Default)]
struct SearchJobPayload{
  jobid:u64,
}
//function to register user
#[ic_cdk::update]
fn register_user(payload: UserPayload) -> Result<User, String> {
    // Validate the payload to ensure that the required fields are present
  
    if payload.username.is_empty()
        ||payload.useremail.is_empty()
        ||payload.phonenumber.is_empty()
    
    {
        return Err("All fields are required".to_string());
    }

    // Validate the payload to ensure that the email format is correct
    if !payload.useremail.contains('@') {
        return Err("enter correct email format".to_string());
    }

    // Ensure email address uniqueness and ownername and also transport name
    let email_exists:bool = USERS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, val)| val.useremail == payload.useremail)
    });
    if email_exists {
        return Err("Email already exists".to_string());
    }

   let username_exists:bool=USERS_STORAGE.with(|storage| {
    storage
        .borrow()
        .iter()
        .any(|(_,val)| val.username == payload.username)
});
if username_exists {
    return Err("The username already exists".to_string());
}
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");
   
    let newuser = User {
        username:payload.username,
        id,
        useremail:payload.useremail,
        phonenumber:payload.phonenumber,
        created_at:time()
       
    };

    USERS_STORAGE.with(|storage| storage.borrow_mut().insert(id, newuser.clone()));

    Ok(newuser)
}


//function where user post a job
#[ic_cdk::update]
fn post_a_job(payload:JobPayload)->Result<Job, String>{

      // Validate the payload to ensure that the required fields are present
      if payload.by.is_empty()
      || payload.companyemail.is_empty()
      || payload.companyname.is_empty()
      ||payload.position.is_empty()
      ||payload.jobname.is_empty()
      ||payload.salary.is_empty()
      ||payload.requirements.is_empty()
      ||payload.applicationdeadeline.is_empty()
       {
          return Err("All fields are required".to_string());
       }
       // Validate the payload to ensure that the email format is correct
    if !payload.companyemail.contains('@') {
        return Err("enter correct email format".to_string());
    }

    //check if user is registered
    let checkuser_exists:bool=USERS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_,val)| val.username == payload.by)
    });
    if !checkuser_exists {
        return Err("You are not registered".to_string());
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");
    let new_job=Job{
        id,
        by:payload.by,
        companyname:payload.companyname,
        companyemail:payload.companyemail,
        companyphonenumber:payload.companyphonenumber,
        position:payload.position,
        jobname:payload.jobname,
        salary:payload.salary,
        requirements:payload.requirements,
        applicationdeadeline:payload.applicationdeadeline,
        created_at:time()
          };

    JOBS_STORAGE.with(|storage| storage.borrow_mut().insert(id, new_job.clone()));

    Ok(new_job)
}

// //Function to retrieve all available jobs
#[ic_cdk::query]
fn get_all_jobs() -> Result<Vec<Job>, String> {

    let jobs = JOBS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .map(|(_, trans)| trans.clone())
            .collect::<Vec<Job>>()
    });

    if  jobs.is_empty() {
        return Err("Currently no jobs available.".to_string());
    }

    else {
        Ok(jobs)
    }

}


// users enquire about a job
#[ic_cdk::update]
fn users_enquire_about_a_job(payload:EnquirePayload)->Result<EnquireAboutAJob,String>{


      // Validate the payload to ensure that the required fields are present
      if payload.by.is_empty()
      || payload.jobname.is_empty()
      ||payload.question.is_empty()
      ||payload.usernamemail.is_empty()
       {
          return Err("All fields are required".to_string());
       }
       // Validate the payload to ensure that the email format is correct
    if !payload.usernamemail.contains('@') {
        return Err("enter correct email format".to_string());
    }
//ensure that the job actually exists
let checkjob_exists:bool=JOBS_STORAGE.with(|storage| {
    storage
        .borrow()
        .iter()
        .any(|(_,val)| val.jobname == payload.jobname)
});
if !checkjob_exists {
    return Err("Job does not exist".to_string());
}
    let id = ID_COUNTER
    .with(|counter| {
        let current_value = *counter.borrow().get();
        counter.borrow_mut().set(current_value + 1)
    })
    .expect("Cannot increment ID counter");
    let new_enquire=EnquireAboutAJob{
    id,
    by:payload.by,
    jobname:payload.jobname,
    question:payload.question,
    usernamemail:payload.usernamemail,
    created_at:time()
     };
QUESTION_STORAGE.with(|storage| storage.borrow_mut().insert(id, new_enquire.clone()));

return Ok(new_enquire);
}

// //users update job status 
#[ic_cdk::update]
fn update_job_details(payload:UpdateJobsPayload)->Result<Job,String>{
     if payload.companyemail.is_empty()
        ||payload.companyname.is_empty()
        || payload.position.is_empty()
        || payload.companyphonenumber.is_empty()
        ||payload.jobname.is_empty()
        ||payload.position.is_empty()
        ||payload.salary.is_empty()
        ||payload.requirements.is_empty()
        ||payload.applicationdeadeline.is_empty()
        ||payload.by.is_empty()
    {
        return Err("Ensure all credentials are provided".to_string());
    }
     // Validate the payload to ensure that the email format is correct
     if !payload.companyemail.contains('@') {
        return Err("Invalid email format".to_string());
    }
    
  
    
match JOBS_STORAGE.with(|service|service.borrow().get(&payload.jobid)){
    Some(mut job)=>{    
                        job.by=payload.by;
                        job.companyname=payload.companyname;
                        job.companyemail=payload.companyemail;
                        job.companyphonenumber=payload.companyphonenumber;
                        job.position=payload.position;
                        job.jobname=payload.jobname;
                        job.salary=payload.salary;
                        job.requirements=payload.requirements;
                        job.applicationdeadeline=payload.applicationdeadeline;
                        do_insert(&job);
                        Ok(job)
                        
    }
    None=>Err("could not update transporter details".to_string()),
}

}

// users search for a job
#[ic_cdk::query]
fn search_for_a_job(payload:SearchJobPayload)->Result<Job,String>{
    let job = JOBS_STORAGE.with(|storage| storage.borrow().get(&payload.jobid));
    match job {
        Some(job) => Ok(job),
        None => Err("job does not exist.".to_string()),
    }
    
}
//owner delete job from josearchers site
#[ic_cdk::update]
  fn delete_job_from_josearchers(payload:DeleteJobPayload)->Result<String,String>{
 //verify its the owner of job is deleteing it
 let checkuser_exists:bool=JOBS_STORAGE.with(|storage| {
    storage
        .borrow()
        .iter()
        .any(|(_,val)| val.by == payload.username)
});
if !checkuser_exists {
    return Err("Job does not exist".to_string());
}
    match JOBS_STORAGE.with(|storage|storage.borrow_mut().remove(&payload.jobid)){
        Some(_val)=>Ok("you have successfully deletede job from jobsearchers".to_string()),
        None=>Err("coulde not delete the job".to_string(),)
    }
  }
//  users comments about jobsearcher platform
  #[ic_cdk::update]
  fn users_commets(payload:CommentPayload)->Result<Comment,String>{
    if payload.by.is_empty()
    ||payload.useremail.is_empty()
    || payload.comment.is_empty()
     {
        return Err("some fields are missing".to_string());
     }
     //check if user is registered
    let checkuser_exists:bool=USERS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_,val)| val.username == payload.by)
    });
    if !checkuser_exists {
        return Err("You are not registered".to_string());
    }
     let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");
     let new_comment=Comment{
        id,
        by:payload.by,
        comment:payload.comment,
        useremail:payload.useremail,
        created_at:time()
     };
     COMMENTS_STORAGE.with(|storage| storage.borrow_mut().insert(id, new_comment.clone()));
     return Ok(new_comment);
  }

//function to get all comments
#[ic_cdk::query]
fn get_all_comments() -> Result<Vec<Comment>, String> {

    let comments = COMMENTS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .map(|(_, trans)| trans.clone())
            .collect::<Vec<Comment>>()
    });

    if  comments.is_empty() {
        return Err("Currently no comments available.".to_string());
    }

    else {
        Ok(comments)
    }

}
//users ask question jobsearcher
#[ic_cdk::update]
  fn users_ask_questions(payload:QuestionPayload)->Result<Question,String>{
    if payload.question.is_empty()
    ||payload.useremail.is_empty()
     {
        return Err("some fields are missing".to_string());
     }
     
     let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");
     let new_question=Question{
        id,
        question:payload.question,
        useremail:payload.useremail,
        created_at:time()
     };
     QUESTIONS_STORAGE.with(|storage| storage.borrow_mut().insert(id, new_question.clone()));
     return Ok(new_question);
  }


//helper unction for updates
fn do_insert(job:&Job){
    JOBS_STORAGE.with(|service|service.borrow_mut().insert(job.id,job.clone()));
}
 ic_cdk::export_candid!();