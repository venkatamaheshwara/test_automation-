use test_automation::add;
#[test]
fn test_correct_ans_fun(){
    
    assert_eq!(add(4,5),9);    
}

#[test]
fn test_wrong_ans_fun(){
    assert_ne!(add(10,20),1);
}