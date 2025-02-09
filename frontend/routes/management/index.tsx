import { Handlers } from "$fresh/server.ts";
import { getCookies } from "$std/http/cookie.ts";
export const handler: Handlers = {
  GET(req, ctx) {
    // TODO: check if user is logged in
    // if user is logged in, redirect to /app
    // else, render the landing page
    
    const cookies = getCookies(req.headers);
    
    if (!Object.keys(cookies).includes("Authorization")) {
      const headers = new Headers();
      headers.append("Location", "/management/login");
      return new Response(null, { status: 302,headers });
    }

    // TODO: verify if the token is still valid
    // if token is invalid, redirect to /management/login

    return ctx.render()
  }
}

export default function Management() {
  return <div>hello management</div>
}