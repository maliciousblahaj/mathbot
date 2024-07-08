/*
case "update_username":
    if len(args) == 1:
        await help(ctx,"account","update_username")
        return
    if authoracc.nextusernameupdate > time.time() and authoracc.admin == False:
        await ctx.send(embed=Embed.ErrorEmbed(ctx.author.id,"usernameupdatetoofast",str(authoracc.nextusernameupdate)))
        return
    if validUsername(args[1]) == False:
        await ctx.send(embed=Embed.ErrorEmbed(ctx.author.id,"invalidusername"))
        return
    if Globals.devdb.usernameExists(args[1]):
        if authoracc.username.lower() == args[1].lower() and authoracc.username != args[1]:
            authoracc.username = args[1]
            await ctx.send(embed=Embed.BaseEmbed(ctx.author.id,title="Successfully updated username", description=f"Successfully set your username to `@{args[1]}`",colorid="success"))
            return
        await ctx.send(embed=Embed.ErrorEmbed(ctx.author.id,"usernamealreadyexists"))
        return
    authoracc.username = args[1]
    await ctx.send(embed=Embed.BaseEmbed(ctx.author.id,title="Successfully updated username", description=f"Successfully set your username to `@{args[1]}`",colorid="success"))
    return
*/