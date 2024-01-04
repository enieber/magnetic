import Dashboard from "layouts/dashboard";
import Profile from "layouts/profile";
import SignIn from "layouts/authentication/sign-in";
import Home from "layouts/home";

import Document from "components/Icons/Document";
import Office from "components/Icons/Office";
import CustomerSupport from "components/Icons/CustomerSupport";

const routes = [
  {
    type: "collapse",
    name: "Home",
    key: "home",
    route: "/",
    icon: <Document size="12px" />,
    component: <Home />,
    noCollapse: true,
  },
  {
    type: "collapse",
    name: "Sign In",
    key: "sign-in",
    route: "/authentication/sign-in",
    icon: <Document size="12px" />,
    component: <SignIn />,
    noCollapse: true,
  },
];

export default routes;
