"use server";
import { Employee, Role } from "@/lib/proto/management";
import { faker } from "@faker-js/faker";

export async function GetEmployees() {
  const arr = [];

  for (let i = 0; i < 25; i++) {
    arr.push({
      id: crypto.randomUUID(),
      userId: crypto.randomUUID(),
      role: Role.EMPLOYEE,
      fullName: faker.person.fullName(),
      address: faker.location.streetAddress(),
      position: faker.person.jobType(),
    });
  }

  // NOTE: simulate loading
  await new Promise((resolve) => setTimeout(resolve, 2000));

  return arr satisfies Employee[];
}
