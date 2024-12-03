import { RouteSectionProps } from "@solidjs/router";

const AdminDashboardLayout = (props: RouteSectionProps) => {
  return <div>{props.children}</div>;
};

export default AdminDashboardLayout;
