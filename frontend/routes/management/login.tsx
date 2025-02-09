import { Handlers } from "$fresh/server.ts";
import { getCookies } from "$std/http/cookie.ts";

export const handler: Handlers = {
  GET(req, ctx) {

    const cookies = getCookies(req.headers);
    
    if (Object.keys(cookies).includes("Authorization")) {
      // TODO: verify if the token is still valid
      const headers = new Headers();
      headers.append("Location", "/management");
      return new Response(null, { status: 302, headers });
    }

    return ctx.render();
  },
  async POST(req, ctx) {
    const form = await req.formData();
    console.log(form.get("email")?.toString());
    console.log(form.get("password")?.toString());

    // TODO: authenticate the user

    // TODO: create a authorization cookie

    return ctx.render();
  },
};

export default function Login() {
  return (
    <form action={'/management/login'} method={'POST'}>
      <input name={'email'} type={'email'} required />
      <input name={'password'} type={'password'} required />
      <button type={"submit"}>Submit</button>
    </form>
  );
}
