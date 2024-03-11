use worker::*;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

  router
    .get_async("/", |_, ctx| async move {
        let kv = ctx.kv("zaitaku")?;
        if let Some(status) = kv.get("status").text().await? {
            return Response::ok(status);
        }
        Response::error("Status not found", 404)
    })
    .post_async("/put_status/:status", |_req, ctx| async move {
        let kv = ctx.kv("zaitaku")?;
        if let Some(status) = ctx.param("status"){
            if status == "true" || status == "false" {
                kv.put("status", status)?.execute().await?;
                return Response::ok("Status updated");
            }
        }
        Response::error("Status not updated", 400)
    })
    .get_async("/robots.txt", |_, _| async {
        Response::ok("User-agent: *\nDisallow: /")
    })
    .run(req, env)
    .await
}


