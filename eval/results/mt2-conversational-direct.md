# MT evaluation

model: `C:\Users\jhvan\AppData\Local\KikoeLive\models\Qwen3.5-4B-Q4_K_M.gguf`  
corpus: `eval/corpus/conversational.jsonl`  
context: true  
spanish route: Direct

## ja-en

| id | Japanese | reference | output | chrF | ms |
|---|---|---|---|---|---|
| c01 | ぺこらは今日ホラーゲームやらないよ。 | Pekora isn't playing a horror game today. | Pekora isn't playing a horror game today. | 100.0 | 644 |
| c02 | え、待って、それ本当？ | Wait, what? Is that true? | Wait, is that true? | 61.5 | 189 |
| c03 | やばいやばいやばい、死ぬ死ぬ！ | Oh no, oh no, oh no, I'm gonna die, I'm gonna die! | Oh no, oh no, oh no, I'm gonna die! | 71.8 | 372 |
| c04 | 全然怖くないし。 | It's not scary at all. | It's not scary at all. | 100.0 | 212 |
| c05 | 怖くないわけじゃないけど、大丈夫。 | It's not that I'm not scared, but I'm okay. | It's not scary, but I'm okay. | 57.2 | 282 |
| c06 | まだ行かないで、ちょっと待って。 | Don't go yet, wait a second. | Don't go yet, wait a little. | 67.1 | 254 |
| c07 | みんな、コメントありがとう！ | Thanks for the comments, everyone! | Thanks everyone for the comments! | 74.8 | 190 |
| c08 | 今日は新しいゲームをやっていくぺこ。 | Today we're going to play a new game, peko. | Today, Pekora is going to play a new game! | 66.4 | 333 |
| c09 | それはマリン船長のせいだよ。 | That's Captain Marine's fault. | That's Captain Marine's fault. | 100.0 | 225 |
| c10 | 誰がそんなこと言ったの？ | Who said that? | Who said that? | 100.0 | 150 |
| c11 | もう二度とやらない。 | I'm never doing this again. | I'm never doing that again. | 77.2 | 213 |
| c12 | あれ、ドアが開かない。 | Huh, the door won't open. | Wait, the door won't open. | 82.7 | 232 |
| c13 | 鍵を取ってから戻ろう。 | Let's get the key and then go back. | Let's go get the key and come back. | 61.5 | 286 |
| c14 | 左だって言ったじゃん！ | I told you it was left! | I said left! | 21.8 | 144 |
| c15 | ちょっと音大きすぎない？ | Isn't the sound a bit too loud? | Is the volume too loud? | 33.2 | 194 |
| c16 | 配信始めるの遅れてごめんね。 | Sorry for starting the stream late. | Sorry, I'm late starting the stream. | 69.5 | 258 |
| c17 | お腹すいたから、ちょっと休憩する。 | I'm hungry, so I'll take a short break. | I'm hungry, so I'll take a short break. | 100.0 | 330 |
| c18 | これ、絶対罠でしょ。 | This is definitely a trap, right? | This is definitely a trap. | 76.0 | 193 |
| c19 | 勝った！やったー！ | I won! Yay! | We won! Yay! | 80.8 | 168 |
| c20 | 負けたくないんだけど。 | I really don't want to lose. | I don't want to lose, though. | 61.6 | 263 |
| c21 | 知らなかった、そんなの。 | I didn't know that. | I didn't know about that. | 74.1 | 219 |
| c22 | それはぺこらのじゃないよ。 | That's not Pekora's. | That's not Pekora's. | 100.0 | 221 |
| c23 | 明日も配信するから、見に来てね。 | I'm streaming tomorrow too, so come watch. | I'll be streaming tomorrow, so come watch! | 72.7 | 288 |
| c24 | え、なんで？意味わかんない。 | Huh, why? That makes no sense. | Wait, why? I don't get it. | 15.8 | 288 |
| c25 | ここで死んだら最初からやり直しだ。 | If I die here, I have to start over from the beginning. | If I die here, I have to start over from the beginning. | 100.0 | 380 |
| c26 | 先輩に怒られちゃった。 | I got scolded by my senpai. | I got scolded by senpai. | 82.1 | 270 |
| c27 | 別に好きじゃないし。 | It's not like I like it or anything. | I don't really like them. | 14.9 | 218 |
| c28 | お疲れぺこ！また明日！ | Good work today, peko! See you tomorrow! | Thanks, peko! See you tomorrow! | 62.9 | 263 |
| c29 | ボス強すぎるんだけど。 | This boss is way too strong. | The boss is way too strong. | 83.4 | 216 |
| c30 | まだ一回も勝ってない。 | I haven't won even once yet. | I haven't won a single time yet. | 52.6 | 263 |
| c31 | スパチャありがとう、助かる！ | Thanks for the superchat, that really helps! | Thanks for the superchat, I appreciate it! | 60.4 | 292 |
| c32 | あ、そこ宝箱あるじゃん。 | Oh, there's a treasure chest right there. | Oh, there's a treasure chest right there. | 100.0 | 290 |
| c33 | 行けると思ったのに。 | I thought I could make it... | I thought I could go. | 60.0 | 196 |
| c34 | 嘘でしょ、また落ちた。 | No way, I fell again. | That's a lie, you fell again. | 51.1 | 267 |
| c35 | 声が聞こえないって？マイク大丈夫？ | You can't hear my voice? Is the mic okay? | You can't hear my voice? Is the mic okay? | 100.0 | 332 |
| c36 | 笑わないでよ！ | Don't laugh! | Don't laugh at me! | 81.4 | 192 |
| c37 | ホロライブの3期生です。 | I'm part of hololive's third generation. | I'm a third-generation hololive member. | 51.3 | 263 |
| c38 | 次は失敗しないから見てて。 | I won't mess up next time, so watch. | Next time I won't fail, so watch this. | 54.0 | 309 |
| c39 | この敵、倒さなくてもいいよ。 | You don't have to defeat this enemy. | You don't have to defeat this enemy. | 100.0 | 261 |
| c40 | えっと、何の話だっけ？ | Um, what were we talking about? | Um, what were we talking about? | 100.0 | 243 |
| p1a | 昨日マリン船長とコラボしたんだけど、 | Yesterday I did a collab with Captain Marine, and... | I collabed with Captain Marine yesterday, but... | 62.7 | 310 |
| p1b | めっちゃ楽しかった。 | It was super fun. | It was so much fun. | 42.2 | 194 |
| p2a | このゲーム、前にもやったことあるんだよね。 | I've actually played this game before. | I've played this game before. | 69.4 | 216 |
| p2b | でも全然覚えてない。 | But I don't remember it at all. | But I don't remember anything about it. | 64.4 | 263 |
| p3a | 犯人はたぶんあの人だと思う。 | I think the culprit is probably that person. | I think the culprit is probably that person. | 100.0 | 264 |
| p3b | いや、違うかも。 | No, maybe not. | No, it might not be him. | 22.1 | 243 |
| p4a | 宿題やった？ | Did you do your homework? | Did you do your homework? | 100.0 | 191 |
| p4b | まだやってない。 | I haven't done it yet. | I haven't done my homework yet. | 64.1 | 242 |

mean chrF 70.9; latency ms median 258 / p95 372 / max 644; load 2399 ms (prepare 15233 ms wall)

## ja-es

| id | Japanese | reference | output | chrF | ms |
|---|---|---|---|---|---|
| c01 | ぺこらは今日ホラーゲームやらないよ。 | Pekora no va a jugar un juego de terror hoy. | Pekora no va a jugar un juego de terror hoy. | 100.0 | 690 |
| c02 | え、待って、それ本当？ | ¿Eh? Espera, ¿eso es verdad? | ¿Eh, espera, ¿eso es verdad? | 85.6 | 312 |
| c03 | やばいやばいやばい、死ぬ死ぬ！ | ¡No, no, no, me muero, me muero! | ¡Ay, ay, ay, me muero, me muero! | 63.4 | 403 |
| c04 | 全然怖くないし。 | No da nada de miedo. | ¡No tiene miedo en absoluto! | 28.0 | 217 |
| c05 | 怖くないわけじゃないけど、大丈夫。 | No es que no tenga miedo, pero estoy bien. | No es que no tenga miedo, pero estoy bien. | 100.0 | 308 |
| c06 | まだ行かないで、ちょっと待って。 | No te vayas todavía, espera un momento. | ¡No vayas todavía, espera un poco! | 64.8 | 286 |
| c07 | みんな、コメントありがとう！ | ¡Gracias por los comentarios a todos! | ¡Gracias a todos por los comentarios! | 86.3 | 264 |
| c08 | 今日は新しいゲームをやっていくぺこ。 | Hoy vamos a jugar un juego nuevo, peko. | ¡Hoy Pekora jugará a un nuevo juego! | 39.1 | 336 |
| c09 | それはマリン船長のせいだよ。 | Eso es culpa de la capitana Marine. | ¡Es culpa del capitán Marine! | 52.0 | 237 |
| c10 | 誰がそんなこと言ったの？ | ¿Quién dijo eso? | ¿Quién dijo eso? | 100.0 | 175 |
| c11 | もう二度とやらない。 | No lo vuelvo a hacer nunca más. | ¡Ya no lo haré otra vez! | 13.7 | 277 |
| c12 | あれ、ドアが開かない。 | ¿Eh? La puerta no se abre. | ¡Eh, la puerta no se abre! | 75.2 | 289 |
| c13 | 鍵を取ってから戻ろう。 | Tomemos la llave y después volvamos. | ¡Primero me quito la llave y luego volveré. | 32.2 | 379 |
| c14 | 左だって言ったじゃん！ | ¡Te dije que era a la izquierda! | ¡Dije que era la izquierda! | 77.9 | 239 |
| c15 | ちょっと音大きすぎない？ | ¿No está el sonido un poco demasiado alto? | ¡Un poco, ¿no es demasiado fuerte el sonido? | 55.2 | 339 |
| c16 | 配信始めるの遅れてごめんね。 | Perdón por empezar tarde la transmisión. | Disculpa, empecé el stream tarde. | 21.2 | 313 |
| c17 | お腹すいたから、ちょっと休憩する。 | Tengo hambre, así que voy a tomar un descansito. | Tengo hambre, así que me daré un pequeño descanso. | 57.0 | 358 |
| c18 | これ、絶対罠でしょ。 | Esto seguro que es una trampa, ¿no? | ¡Esto, definitivamente es una trampa! | 42.0 | 293 |
| c19 | 勝った！やったー！ | ¡Gané! ¡Bien! | ¡Ganamos! ¡Qué bien! | 40.2 | 267 |
| c20 | 負けたくないんだけど。 | Es que no quiero perder. | No quiero perder, pero... | 65.6 | 194 |
| c21 | 知らなかった、そんなの。 | No sabía eso. | ¡No lo sabía, eso! | 47.4 | 217 |
| c22 | それはぺこらのじゃないよ。 | Eso no es de Pekora. | Eso no es de Pekora. | 100.0 | 244 |
| c23 | 明日も配信するから、見に来てね。 | Mañana también voy a transmitir, así que vengan a ver. | Mañana también haré una transmisión, así que ven a verla. | 61.3 | 408 |
| c24 | え、なんで？意味わかんない。 | ¿Eh, por qué? No tiene sentido. | ¿Eh, ¿por qué? No entiendo el significado. | 47.3 | 391 |
| c25 | ここで死んだら最初からやり直しだ。 | Si muero aquí, tengo que empezar de nuevo desde el principio. | ¡Si muero aquí, tengo que empezar de nuevo! | 69.2 | 340 |
| c26 | 先輩に怒られちゃった。 | Mi senpai me regañó. | ¡El senpai me enojó. | 41.0 | 272 |
| c27 | 別に好きじゃないし。 | Tampoco es que me guste. | No me gusta nada. | 24.2 | 173 |
| c28 | お疲れぺこ！また明日！ | ¡Buen trabajo, peko! ¡Hasta mañana! | ¡Gracias, Pekora! ¡Mañana otra vez! | 30.7 | 359 |
| c29 | ボス強すぎるんだけど。 | Este jefe es demasiado fuerte. | ¡El boss es demasiado fuerte! | 63.3 | 218 |
| c30 | まだ一回も勝ってない。 | Todavía no he ganado ni una vez. | ¡Aún no he ganado ni una vez. | 72.6 | 289 |
| c31 | スパチャありがとう、助かる！ | ¡Gracias por el superchat, me ayuda mucho! | ¡Gracias por el superchat! ¡Me salvó! | 61.9 | 354 |
| c32 | あ、そこ宝箱あるじゃん。 | Ah, ahí hay un cofre del tesoro. | ¡Ah, hay un cofre ahí, ¿no? | 41.5 | 339 |
| c33 | 行けると思ったのに。 | Pensé que lo iba a lograr... | ¡Pensé que podía ir. | 32.6 | 237 |
| c34 | 嘘でしょ、また落ちた。 | No puede ser, me caí otra vez. | ¡Miente! ¡Caí otra vez! | 38.7 | 289 |
| c35 | 声が聞こえないって？マイク大丈夫？ | ¿Que no se escucha mi voz? ¿Está bien el micrófono? | ¿No se oye tu voz? ¿El micrófono está bien? | 50.9 | 428 |
| c36 | 笑わないでよ！ | ¡No se rían! | ¡No rías! | 26.0 | 174 |
| c37 | ホロライブの3期生です。 | Soy de la tercera generación de hololive. | Soy de la tercera generación de Hololive. | 100.0 | 289 |
| c38 | 次は失敗しないから見てて。 | La próxima no voy a fallar, así que miren. | La próxima no fallaré, así que mirad. | 63.4 | 310 |
| c39 | この敵、倒さなくてもいいよ。 | No hace falta derrotar a este enemigo. | ¡Este enemigo, no es necesario derrotarlo! | 49.5 | 295 |
| c40 | えっと、何の話だっけ？ | Eh... ¿de qué estábamos hablando? | Eh, ¿de qué estábamos hablando? | 84.1 | 295 |
| p1a | 昨日マリン船長とコラボしたんだけど、 | Ayer hice una colaboración con la capitana Marine, y... | ¡Ay! ¡Ayer hice un collab con el capitán Marine, pero! | 41.5 | 477 |
| p1b | めっちゃ楽しかった。 | Fue súper divertido. | ¡Fue súper divertido! | 92.3 | 202 |
| p2a | このゲーム、前にもやったことあるんだよね。 | Ya jugué este juego antes, ¿sabes? | ¡Este juego ya lo he jugado antes, ¿verdad? | 48.5 | 381 |
| p2b | でも全然覚えてない。 | Pero no me acuerdo para nada. | Pero no recuerdo nada de nada. | 44.9 | 225 |
| p3a | 犯人はたぶんあの人だと思う。 | Creo que el culpable probablemente es esa persona. | Creo que el culpable es probablemente esa persona. | 87.3 | 321 |
| p3b | いや、違うかも。 | No, tal vez no. | No, quizás no sea esa persona. | 15.3 | 245 |
| p4a | 宿題やった？ | ¿Hiciste la tarea? | ¿Hiciste la tarea? | 100.0 | 225 |
| p4b | まだやってない。 | Todavía no la hice. | Aún no lo he hecho. | 13.0 | 226 |

mean chrF 57.2; latency ms median 289 / p95 428 / max 690; load 0 ms (prepare 14404 ms wall)

