# MT evaluation

model: `C:\Users\jhvan\AppData\Local\KikoeLive\models\Qwen3.5-4B-Q4_K_M.gguf`  
corpus: `eval/corpus/conversational.jsonl`  
context: true

## ja-en

| id | Japanese | reference | output | chrF | ms |
|---|---|---|---|---|---|
| c01 | ぺこらは今日ホラーゲームやらないよ。 | Pekora isn't playing a horror game today. | Pekora isn't playing a horror game today. | 100.0 | 658 |
| c02 | え、待って、それ本当？ | Wait, what? Is that true? | Wait, is that real? | 36.9 | 194 |
| c03 | やばいやばいやばい、死ぬ死ぬ！ | Oh no, oh no, oh no, I'm gonna die, I'm gonna die! | Oh no, oh no, oh no, I'm gonna die! | 71.8 | 384 |
| c04 | 全然怖くないし。 | It's not scary at all. | It's not scary at all. | 100.0 | 216 |
| c05 | 怖くないわけじゃないけど、大丈夫。 | It's not that I'm not scared, but I'm okay. | It's not scary, but I'm okay. | 57.2 | 285 |
| c06 | まだ行かないで、ちょっと待って。 | Don't go yet, wait a second. | Don't go yet, wait a sec. | 83.5 | 262 |
| c07 | みんな、コメントありがとう！ | Thanks for the comments, everyone! | Thanks everyone for the comments! | 74.8 | 189 |
| c08 | 今日は新しいゲームをやっていくぺこ。 | Today we're going to play a new game, peko. | Today, Pekora is going to play a new game! | 66.4 | 334 |
| c09 | それはマリン船長のせいだよ。 | That's Captain Marine's fault. | That's Captain Marine's fault. | 100.0 | 218 |
| c10 | 誰がそんなこと言ったの？ | Who said that? | Who said that? | 100.0 | 147 |
| c11 | もう二度とやらない。 | I'm never doing this again. | I'm never doing that again. | 77.2 | 218 |
| c12 | あれ、ドアが開かない。 | Huh, the door won't open. | Wait, the door won't open. | 82.7 | 237 |
| c13 | 鍵を取ってから戻ろう。 | Let's get the key and then go back. | Let's go back and get the key. | 57.2 | 264 |
| c14 | 左だって言ったじゃん！ | I told you it was left! | I said left! | 21.8 | 146 |
| c15 | ちょっと音大きすぎない？ | Isn't the sound a bit too loud? | Is the volume too loud? | 33.2 | 193 |
| c16 | 配信始めるの遅れてごめんね。 | Sorry for starting the stream late. | Sorry, my stream started a bit late. | 40.3 | 261 |
| c17 | お腹すいたから、ちょっと休憩する。 | I'm hungry, so I'll take a short break. | I'm hungry, so I'll take a short break. | 100.0 | 333 |
| c18 | これ、絶対罠でしょ。 | This is definitely a trap, right? | This is definitely a trap. | 76.0 | 191 |
| c19 | 勝った！やったー！ | I won! Yay! | We won! Yay! | 80.8 | 170 |
| c20 | 負けたくないんだけど。 | I really don't want to lose. | I don't want to lose. | 70.5 | 214 |
| c21 | 知らなかった、そんなの。 | I didn't know that. | I didn't know that. | 100.0 | 192 |
| c22 | それはぺこらのじゃないよ。 | That's not Pekora's. | That's not Pekora's. | 100.0 | 214 |
| c23 | 明日も配信するから、見に来てね。 | I'm streaming tomorrow too, so come watch. | I'll be streaming tomorrow, so come watch! | 72.7 | 285 |
| c24 | え、なんで？意味わかんない。 | Huh, why? That makes no sense. | Wait, why? I don't get it. | 15.8 | 289 |
| c25 | ここで死んだら最初からやり直しだ。 | If I die here, I have to start over from the beginning. | If I die here, I have to start over. | 64.3 | 312 |
| c26 | 先輩に怒られちゃった。 | I got scolded by my senpai. | I got scolded by senpai. | 82.1 | 267 |
| c27 | 別に好きじゃないし。 | It's not like I like it or anything. | I don't really like them. | 14.9 | 218 |
| c28 | お疲れぺこ！また明日！ | Good work today, peko! See you tomorrow! | Thanks, peko! See you tomorrow! | 62.9 | 266 |
| c29 | ボス強すぎるんだけど。 | This boss is way too strong. | The boss is way too strong. | 83.4 | 217 |
| c30 | まだ一回も勝ってない。 | I haven't won even once yet. | I haven't won a single time yet. | 52.6 | 265 |
| c31 | スパチャありがとう、助かる！ | Thanks for the superchat, that really helps! | Thanks for the superchat, I'm saved! | 58.9 | 285 |
| c32 | あ、そこ宝箱あるじゃん。 | Oh, there's a treasure chest right there. | Oh, there's a treasure chest right there. | 100.0 | 291 |
| c33 | 行けると思ったのに。 | I thought I could make it... | I thought I could go. | 60.0 | 198 |
| c34 | 嘘でしょ、また落ちた。 | No way, I fell again. | That's a lie, you fell again. | 51.1 | 270 |
| c35 | 声が聞こえないって？マイク大丈夫？ | You can't hear my voice? Is the mic okay? | You can't hear the voice? Is the mic okay? | 84.3 | 344 |
| c36 | 笑わないでよ！ | Don't laugh! | Don't laugh at me! | 81.4 | 193 |
| c37 | ホロライブの3期生です。 | I'm part of hololive's third generation. | I'm from Hololive Generation 3. | 46.6 | 265 |
| c38 | 次は失敗しないから見てて。 | I won't mess up next time, so watch. | Next time, I won't fail, so watch this. | 56.8 | 340 |
| c39 | この敵、倒さなくてもいいよ。 | You don't have to defeat this enemy. | You don't even have to defeat this enemy. | 87.9 | 288 |
| c40 | えっと、何の話だっけ？ | Um, what were we talking about? | Um, what were we talking about? | 100.0 | 244 |
| p1a | 昨日マリン船長とコラボしたんだけど、 | Yesterday I did a collab with Captain Marine, and... | I collabed with Captain Marine yesterday, but... | 62.7 | 312 |
| p1b | めっちゃ楽しかった。 | It was super fun. | It was so much fun. | 42.2 | 203 |
| p2a | このゲーム、前にもやったことあるんだよね。 | I've actually played this game before. | I've played this game before. | 69.4 | 224 |
| p2b | でも全然覚えてない。 | But I don't remember it at all. | But I don't remember anything at all. | 76.6 | 268 |
| p3a | 犯人はたぶんあの人だと思う。 | I think the culprit is probably that person. | I think the culprit is probably that person. | 100.0 | 264 |
| p3b | いや、違うかも。 | No, maybe not. | No, maybe not. | 100.0 | 174 |
| p4a | 宿題やった？ | Did you do your homework? | Did you do your homework? | 100.0 | 196 |
| p4b | まだやってない。 | I haven't done it yet. | Not yet. | 19.6 | 128 |

mean chrF 70.3; latency ms median 261 / p95 344 / max 658; load 3180 ms (prepare 15420 ms wall)

## ja-es

| id | Japanese | reference | output | chrF | ms |
|---|---|---|---|---|---|
| c01 | ぺこらは今日ホラーゲームやらないよ。 | Pekora no va a jugar un juego de terror hoy. | Pekora no juega a juegos de terror hoy. | 60.1 | 617 |
| c02 | え、待って、それ本当？ | ¿Eh? Espera, ¿eso es verdad? | ¿Eh, espera, ¿eso es verdad? | 85.6 | 312 |
| c03 | やばいやばいやばい、死ぬ死ぬ！ | ¡No, no, no, me muero, me muero! | ¡Ay, ay, ay, me muero, me muero! | 63.4 | 406 |
| c04 | 全然怖くないし。 | No da nada de miedo. | ¡No tiene miedo en absoluto! | 28.0 | 219 |
| c05 | 怖くないわけじゃないけど、大丈夫。 | No es que no tenga miedo, pero estoy bien. | No es que no tenga miedo, pero estoy bien. | 100.0 | 313 |
| c06 | まだ行かないで、ちょっと待って。 | No te vayas todavía, espera un momento. | No vayas todavía, espera un poco. | 66.3 | 262 |
| c07 | みんな、コメントありがとう！ | ¡Gracias por los comentarios a todos! | ¡Gracias a todos por los comentarios! | 86.3 | 263 |
| c08 | 今日は新しいゲームをやっていくぺこ。 | Hoy vamos a jugar un juego nuevo, peko. | ¡Hoy Pekora jugará a un nuevo juego! | 39.1 | 331 |
| c09 | それはマリン船長のせいだよ。 | Eso es culpa de la capitana Marine. | ¡Es culpa del capitán Marine! | 52.0 | 241 |
| c10 | 誰がそんなこと言ったの？ | ¿Quién dijo eso? | ¿Quién dijo eso? | 100.0 | 166 |
| c11 | もう二度とやらない。 | No lo vuelvo a hacer nunca más. | ¡Ya no lo haré otra vez. | 14.4 | 261 |
| c12 | あれ、ドアが開かない。 | ¿Eh? La puerta no se abre. | ¡Eh, la puerta no se abre. | 80.6 | 286 |
| c13 | 鍵を取ってから戻ろう。 | Tomemos la llave y después volvamos. | ¡Vamos a volver a recoger la llave. | 31.5 | 289 |
| c14 | 左だって言ったじゃん！ | ¡Te dije que era a la izquierda! | ¡Dije que era de la izquierda! | 73.6 | 275 |
| c15 | ちょっと音大きすぎない？ | ¿No está el sonido un poco demasiado alto? | ¿Un poco, ¿no es demasiado fuerte el sonido? | 55.2 | 332 |
| c16 | 配信始めるの遅れてごめんね。 | Perdón por empezar tarde la transmisión. | Disculpa, tardé en empezar la transmisión. | 55.0 | 317 |
| c17 | お腹すいたから、ちょっと休憩する。 | Tengo hambre, así que voy a tomar un descansito. | Tengo hambre, así que me daré un pequeño descanso. | 57.0 | 361 |
| c18 | これ、絶対罠でしょ。 | Esto seguro que es una trampa, ¿no? | ¡Esto, definitivamente es una trampa! | 42.0 | 288 |
| c19 | 勝った！やったー！ | ¡Gané! ¡Bien! | ¡Ganamos! ¡Qué bien! | 40.2 | 271 |
| c20 | 負けたくないんだけど。 | Es que no quiero perder. | No quiero perder, pero... | 65.6 | 192 |
| c21 | 知らなかった、そんなの。 | No sabía eso. | ¡No lo sabía, eso! | 47.4 | 219 |
| c22 | それはぺこらのじゃないよ。 | Eso no es de Pekora. | Eso no es de Pekora. | 100.0 | 241 |
| c23 | 明日も配信するから、見に来てね。 | Mañana también voy a transmitir, así que vengan a ver. | Mañana también haré una transmisión, así que ven a verla. | 61.3 | 404 |
| c24 | え、なんで？意味わかんない。 | ¿Eh, por qué? No tiene sentido. | ¿Eh, ¿por qué? No entiendo el significado. | 47.3 | 381 |
| c25 | ここで死んだら最初からやり直しだ。 | Si muero aquí, tengo que empezar de nuevo desde el principio. | Si muero aquí, tengo que empezar de nuevo. | 70.2 | 310 |
| c26 | 先輩に怒られちゃった。 | Mi senpai me regañó. | ¡El senpai me enojó. | 41.0 | 264 |
| c27 | 別に好きじゃないし。 | Tampoco es que me guste. | No, no me gusta nada. | 24.2 | 216 |
| c28 | お疲れぺこ！また明日！ | ¡Buen trabajo, peko! ¡Hasta mañana! | ¡Gracias Pekora! ¡Mañana otra vez! | 28.1 | 333 |
| c29 | ボス強すぎるんだけど。 | Este jefe es demasiado fuerte. | ¡El boss es demasiado fuerte! | 63.3 | 219 |
| c30 | まだ一回も勝ってない。 | Todavía no he ganado ni una vez. | ¡Aún no hemos ganado ni una vez. | 60.9 | 285 |
| c31 | スパチャありがとう、助かる！ | ¡Gracias por el superchat, me ayuda mucho! | ¡Gracias por el superchat, me salvó! | 70.1 | 338 |
| c32 | あ、そこ宝箱あるじゃん。 | Ah, ahí hay un cofre del tesoro. | ¡Ah, hay un cofre ahí, ¿no? | 41.5 | 331 |
| c33 | 行けると思ったのに。 | Pensé que lo iba a lograr... | ¡Pensé que podía ir. | 32.6 | 240 |
| c34 | 嘘でしょ、また落ちた。 | No puede ser, me caí otra vez. | ¡Miente, ¡caí de nuevo! | 14.6 | 288 |
| c35 | 声が聞こえないって？マイク大丈夫？ | ¿Que no se escucha mi voz? ¿Está bien el micrófono? | ¿No se oye tu voz? ¿El micrófono está bien? | 50.9 | 434 |
| c36 | 笑わないでよ！ | ¡No se rían! | ¡No reías! | 23.1 | 174 |
| c37 | ホロライブの3期生です。 | Soy de la tercera generación de hololive. | Soy de la tercera generación de Hololive. | 100.0 | 286 |
| c38 | 次は失敗しないから見てて。 | La próxima no voy a fallar, así que miren. | La próxima no fallará, así que mirad. | 63.4 | 313 |
| c39 | この敵、倒さなくてもいいよ。 | No hace falta derrotar a este enemigo. | ¡Este enemigo, no es necesario derrotarlo. | 50.5 | 291 |
| c40 | えっと、何の話だっけ？ | Eh... ¿de qué estábamos hablando? | Eh, ¿de qué estábamos hablando? | 84.1 | 290 |
| p1a | 昨日マリン船長とコラボしたんだけど、 | Ayer hice una colaboración con la capitana Marine, y... | Ayer hice un collab con el capitán Marine, pero... | 43.6 | 387 |
| p1b | めっちゃ楽しかった。 | Fue súper divertido. | ¡Fue súper divertido! | 92.3 | 202 |
| p2a | このゲーム、前にもやったことあるんだよね。 | Ya jugué este juego antes, ¿sabes? | ¡Este juego ya lo jugué antes, ¿verdad? | 51.8 | 354 |
| p2b | でも全然覚えてない。 | Pero no me acuerdo para nada. | Pero no recuerdo nada. | 46.0 | 177 |
| p3a | 犯人はたぶんあの人だと思う。 | Creo que el culpable probablemente es esa persona. | Creo que el culpable es probablemente esa persona. | 87.3 | 309 |
| p3b | いや、違うかも。 | No, tal vez no. | No, quizás no sea esa persona. | 15.3 | 244 |
| p4a | 宿題やった？ | ¿Hiciste la tarea? | ¿Hiciste la tarea? | 100.0 | 221 |
| p4b | まだやってない。 | Todavía no la hice. | Aún no la he hecho. | 22.7 | 216 |

mean chrF 56.9; latency ms median 288 / p95 406 / max 617; load 0 ms (prepare 13969 ms wall)

