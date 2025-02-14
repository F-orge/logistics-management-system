import React from "react";

export default function Layout(
  { create, read }: {
    create: React.ReactNode;
    read: React.ReactNode;
  },
) {
  return (
    <article>
      <section>
        <h2 className="scroll-m-20 border-b pb-2 text-3xl font-semibold tracking-tight first:mt-0">
          Employees
        </h2>
      </section>
      <section className="flex flex-row gap-5 justify-end py-4">
        {create}
      </section>
      <section className="pb-4">
        {read}
      </section>
    </article>
  );
}
