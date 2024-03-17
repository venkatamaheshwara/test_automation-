use test_automation::add;
use test_automation::sub;
#[test]
fn test_add_correct_ans_fun(){
    
    assert_eq!(add(4,5),9);   
     
}

#[test]
fn test_sub_correct_ans_fun(){
    assert_eq!(sub(2,1),1);
}

#[test]
fn test_wrong_ans_fun(){
    assert_ne!(add(10,20),1);
}

#[test]
fn test_sub_wrong_ans_fun(){
    assert_ne!(sub(2,1),0);
}

