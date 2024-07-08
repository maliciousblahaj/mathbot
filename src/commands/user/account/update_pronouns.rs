/*
case "update_pronouns":
    if len(args) == 1:
        await help(ctx,"account","update_pronouns")
        return
    if args[1] in ["remove", "delete"]:
        authoracc.pronouns = None
        await ctx.send(embed=Embed.BaseEmbed(ctx.author.id,title="Successfully removed pronouns", description=f"Successfully removed your pronouns",colorid="success"))
        return
    pronouns = args[1]
    if validPronouns(pronouns) == False:
        await ctx.send(embed=Embed.ErrorEmbed(ctx.author.id,"invalidpronouns"))
        return
    authoracc.pronouns = pronouns
    await ctx.send(embed=Embed.BaseEmbed(ctx.author.id,title="Successfully updated pronouns", description=f"Successfully updated your pronouns to `{pronouns}`",colorid="success"))
    return
*/