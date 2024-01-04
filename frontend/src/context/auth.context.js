import React from 'react';
import PropTypes from 'prop-types';
import AuthApi from '../api/auth';
import { useNavigate } from "react-router-dom";

const AuthContext = React.createContext(null);

export const AuthProvider = ({ userData, children }) => {
  let [user, setUser] = React.useState(userData);
  let [error, setError] = React.useState("");
  const navigate = useNavigate();
  
  const login = async (event, email, password) => {
    if (event) {
      event.preventDefault();
    }

    //handle exceptions like: no email entered, no password entered, here.
    try {
      let response = await AuthApi.Login({
        email,
        password,
      });
    
      if (response.data && response.data.success === false) {
        //display error coming from server
        return setError(response.data.msg);
      }
    
      const { token, ...rest } = response.data;
      setUser(rest);
     
      localStorage.setItem("token", token);
    } catch (err) {
      //display error originating from server / other sources
      console.log(err);
      if (err.response) {
        return setError(err.response.data.msg);
      }
      return setError("There has been an error.");
    }
  };
  
  React.useEffect(() => {
    const token = localStorage.getItem("token")
    if (token) {
      AuthApi.Info()
        .then(response => {          
          setUser(response.data)
        })
      .catch(err => {
        setError(err.message);
      });

 //      setUser(JSON.parse(sa));
    }
  }, [])
  
  return (
    <AuthContext.Provider value={{ user, login }}>
      {children}
    </AuthContext.Provider>
  );
};


AuthProvider.propTypes = {
  userData: PropTypes.object,
  children: PropTypes.node,
}
 
export const useAuth = () => React.useContext(AuthContext);

