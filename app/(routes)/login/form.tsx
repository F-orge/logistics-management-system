"use client";
import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { loginAction } from "@/actions/auth";
import { useActionState, useEffect } from "react";
import { toast } from "sonner";
import { Loader2, X } from "lucide-react";

export function LoginForm({
  className,
  ...props
}: React.ComponentPropsWithoutRef<"form">) {
  const [state, formAction, pending] = useActionState(loginAction, {
    message: "",
  });

  useEffect(() => {
    if (state.message !== "") {
      toast("Server response", {
        description: state.message,
        action: {
          label: <X size={16} />,
          onClick: () => {},
        },
      });
    }
  }, [state]);

  return (
    <div>
      <form
        action={formAction}
        className={cn("flex flex-col gap-6", className)}
        {...props}
      >
        <div className="flex flex-col items-center gap-2 text-center">
          <h1 className="text-2xl font-bold">Login to your account</h1>
          <p className="text-balance text-sm text-muted-foreground">
            Enter your email below to login to your account
          </p>
        </div>
        <div className="grid gap-6">
          <div className="grid gap-2">
            <Label htmlFor="email">Email</Label>
            <Input
              disabled={pending}
              id="email"
              type="email"
              name="email"
              placeholder="abc@example.com"
              required
            />
          </div>
          <div className="grid gap-2">
            <div className="flex items-center">
              <Label htmlFor="password">Password</Label>
              <a
                href="#"
                className="ml-auto text-sm underline-offset-4 hover:underline"
              >
                Forgot your password?
              </a>
            </div>
            <Input
              disabled={pending}
              id="password"
              type="password"
              name="password"
              required
            />
          </div>
          <Button disabled={pending} type="submit" className="w-full">
            {pending
              ? (
                <>
                  <Loader2 size={16} className="animate-spin" />
                  <span>Loading</span>
                </>
              )
              : (
                <>
                  <span>Login</span>
                </>
              )}
          </Button>
        </div>
      </form>
    </div>
  );
}
