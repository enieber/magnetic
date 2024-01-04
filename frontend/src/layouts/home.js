// @mui material components
import Grid from "@mui/material/Grid";
import Card from "@mui/material/Card";
import AppBar from "@mui/material/AppBar";
import { useNavigate } from "react-router-dom";


// @mui icons
import FacebookIcon from "@mui/icons-material/Facebook";
import TwitterIcon from "@mui/icons-material/Twitter";
import InstagramIcon from "@mui/icons-material/Instagram";

// Soft UI Dashboard React components
import SoftBox from "components/SoftBox";
import SoftTypography from "components/SoftTypography";
import SoftButton from "components/SoftButton";
import Toolbar from '@mui/material/Toolbar';


import Footer from "layouts/Footer";

// Overview page components
import Header from "layouts/profile/components/Header";
import PlatformSettings from "layouts/profile/components/PlatformSettings";

// Data
import profilesListData from "layouts/profile/data/profilesListData";

// Images
import homeDecor1 from "assets/images/home-decor-1.jpg";
import homeDecor2 from "assets/images/home-decor-2.jpg";
import homeDecor3 from "assets/images/home-decor-3.jpg";
import team1 from "assets/images/team-1.jpg";
import team2 from "assets/images/team-2.jpg";
import team3 from "assets/images/team-3.jpg";
import team4 from "assets/images/team-4.jpg";

function Home() {
    const navigate = useNavigate();
  return (
<div>
   <AppBar
      color="inherit"

    >
        <Toolbar>

        <SoftTypography    variant="h1" component="div" sx={{ flexGrow: 1 }}>
    Popsolutions Cloud
  </SoftTypography>

        <SoftBox mt={4} mb={1}>          <SoftButton variant="gradient" color="info" fullWidth onClick={() => navigate("/authentication/sign-in") }>
            Login
          </SoftButton>
  </SoftBox>
  </Toolbar>
  </AppBar>
      <SoftBox mt={5} mb={3}>
        <Grid container spacing={3}>
          <Grid item xs={12} md={6} xl={4}>
          </Grid>
          <Grid item xs={12} md={6} xl={4}>
          </Grid>
          <Grid item xs={12} xl={4}>
          </Grid>
        </Grid>
      </SoftBox>
      <Footer />

    </div>
  );
}

export default Home;
