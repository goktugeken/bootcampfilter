
struct FilterCondition<T>{
    filter:T,

}

impl<T: PartialOrd> FilterCondition<T>{
fn is_match(&self,item:&T) ->bool {
    item > &self.filter
}
}
fn customfilter<T>(list:Vec<T>,condition:&FilterCondition<T>)-> Vec<T> where T:PartialOrd
{
return list.into_iter().filter(|item:&T|condition.is_match(item)).collect();

}

fn main() {

let numbers=vec![5,6,7,8,9,10,15,20,25,30,35];
let condition=FilterCondition{filter: 10 as u64} ;


let filteredlist = customfilter(numbers, &condition);
println!("{:?}",filteredlist);


}

