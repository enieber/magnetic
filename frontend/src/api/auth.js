import axios from "./index";

class AuthApi {

  static Login = (data) => {
    return axios.post(`/auth/login`, data);
  };
  
  static Info = () => {
    return axios.get(`/user/current`);    
  };
}

export default AuthApi;
