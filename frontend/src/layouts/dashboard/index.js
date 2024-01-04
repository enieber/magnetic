import Grid from "@mui/material/Grid";
import Icon from "@mui/material/Icon";

// Soft UI Dashboard React components
import SoftBox from "components/SoftBox";
import SoftTypography from "components/SoftTypography";
import Footer from "layouts/Footer";
import DashboardNavbar from "layouts/DashBoardNavbar"
import DashboardLayout from "layouts/DashboardLayout";

function Dashboard() {
  return (
    <DashboardLayout>
      <DashboardNavbar />
      <Footer />
    </DashboardLayout>
  );
}

export default Dashboard;
