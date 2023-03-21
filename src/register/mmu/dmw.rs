pub trait Dmw{
    fn plv0(&self)->bool;
    fn set_plv0(&mut self,plv0:bool)->&mut Self;
    fn plv1(&self)->bool;
    fn set_plv1(&mut self,plv1:bool)->&mut Self;
    fn plv2(&self)->bool;
    fn set_plv2(&mut self,plv2:bool)->&mut Self;
    fn plv3(&self)->bool;
    fn set_plv3(&mut self,plv3:bool)->&mut Self;
    fn mat(&self)->usize;
    fn set_mat(&mut self,mat:usize)->&mut Self;
    fn vseg(&self)->usize;
    fn set_vesg(&mut self,vseg:usize)->&mut Self;
}