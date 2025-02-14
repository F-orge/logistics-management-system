import { GetEmployees } from "@/actions/human-resource/employee";
import {
  Table,
  TableBody,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import EmployeeInfoDialog from "./dialog";

export default async function Page() {
  const employees = await GetEmployees();
  return (
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead>Full Name</TableHead>
          <TableHead>Position</TableHead>
          <TableHead>Role</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {employees.map((employee, index) => (
          <EmployeeInfoDialog
            key={`employee-${index}`}
            employee={employee}
          />
        ))}
      </TableBody>
    </Table>
  );
}
