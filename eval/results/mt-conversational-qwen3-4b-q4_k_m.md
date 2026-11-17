# MT evaluation

model: `C:\Users\jhvan\AppData\Local\KikoeLive\models\Qwen3-4B-Q4_K_M.gguf`  
corpus: `eval/corpus/conversational.jsonl`  
context: true

## ja-en

| id | Japanese | reference | output | chrF | ms |
|---|---|---|---|---|---|
| c01 | ぺこらは今日ホラーゲームやらないよ。 | Pekora isn't playing a horror game today. | Pekora isn't playing a horror game today. | 100.0 | 493 |
| c02 | え、待って、それ本当？ | Wait, what? Is that true? | Wait, is that really true? | 44.1 | 166 |
| c03 | やばいやばいやばい、死ぬ死ぬ！ | Oh no, oh no, oh no, I'm gonna die, I'm gonna die! | Ugh! It's so bad, it's so bad, it's so bad! I'm going to die! | 18.3 | 468 |
| c04 | 全然怖くないし。 | It's not scary at all. | Far from being scary. | 19.7 | 129 |
| c05 | 怖くないわけじゃないけど、大丈夫。 | It's not that I'm not scared, but I'm okay. | It's not scary, but it's okay. | 42.7 | 223 |
| c06 | まだ行かないで、ちょっと待って。 | Don't go yet, wait a second. | Wait a minute, don't go yet. | 50.6 | 205 |
| c07 | みんな、コメントありがとう！ | Thanks for the comments, everyone! | Everyone, thanks for the comments! | 84.3 | 171 |
| c08 | 今日は新しいゲームをやっていくぺこ。 | Today we're going to play a new game, peko. | Today, I'm going to play a new game, peko. | 77.5 | 303 |
| c09 | それはマリン船長のせいだよ。 | That's Captain Marine's fault. | That's Marine Captain's fault. | 67.8 | 172 |
| c10 | 誰がそんなこと言ったの？ | Who said that? | Who said that? | 100.0 | 118 |
| c11 | もう二度とやらない。 | I'm never doing this again. | I won't do it again. | 25.5 | 168 |
| c12 | あれ、ドアが開かない。 | Huh, the door won't open. | The door won't open. | 81.8 | 150 |
| c13 | 鍵を取ってから戻ろう。 | Let's get the key and then go back. | Take the key and go back. | 47.2 | 167 |
| c14 | 左だって言ったじゃん！ | I told you it was left! | Left, I said that! | 18.9 | 149 |
| c15 | ちょっと音大きすぎない？ | Isn't the sound a bit too loud? | Hmm, the sound's a bit too loud. | 62.2 | 220 |
| c16 | 配信始めるの遅れてごめんね。 | Sorry for starting the stream late. | Sorry for starting the stream late. | 100.0 | 165 |
| c17 | お腹すいたから、ちょっと休憩する。 | I'm hungry, so I'll take a short break. | I'm hungry, so I'll take a break for a bit. | 72.4 | 298 |
| c18 | これ、絶対罠でしょ。 | This is definitely a trap, right? | This is definitely a trap. | 76.0 | 149 |
| c19 | 勝った！やったー！ | I won! Yay! | Won! Yay! | 86.3 | 112 |
| c20 | 負けたくないんだけど。 | I really don't want to lose. | I don't want to lose. | 70.5 | 164 |
| c21 | 知らなかった、そんなの。 | I didn't know that. | I didn't know, that's it. | 70.5 | 206 |
| c22 | それはぺこらのじゃないよ。 | That's not Pekora's. | That's not Pekora's. | 100.0 | 182 |
| c23 | 明日も配信するから、見に来てね。 | I'm streaming tomorrow too, so come watch. | We'll be streaming tomorrow, so come watch. | 74.3 | 221 |
| c24 | え、なんで？意味わかんない。 | Huh, why? That makes no sense. | What's the deal? I don't get it. | 13.4 | 239 |
| c25 | ここで死んだら最初からやり直しだ。 | If I die here, I have to start over from the beginning. | If I die here, I'll have to start over. | 56.7 | 258 |
| c26 | 先輩に怒られちゃった。 | I got scolded by my senpai. | Got scolded by the senpai. | 70.8 | 200 |
| c27 | 別に好きじゃないし。 | It's not like I like it or anything. | I don't really like it. | 20.7 | 167 |
| c28 | お疲れぺこ！また明日！ | Good work today, peko! See you tomorrow! | Pekora, you're tired! See you tomorrow! | 51.7 | 256 |
| c29 | ボス強すぎるんだけど。 | This boss is way too strong. | The boss is way too strong. | 83.4 | 166 |
| c30 | まだ一回も勝ってない。 | I haven't won even once yet. | We haven't won even once. | 78.4 | 166 |
| c31 | スパチャありがとう、助かる！ | Thanks for the superchat, that really helps! | Thanks to superchat, it's helpful! | 39.9 | 205 |
| c32 | あ、そこ宝箱あるじゃん。 | Oh, there's a treasure chest right there. | Oh, there's a treasure chest right there. | 100.0 | 220 |
| c33 | 行けると思ったのに。 | I thought I could make it... | I thought we could go. | 41.3 | 149 |
| c34 | 嘘でしょ、また落ちた。 | No way, I fell again. | Lie, it's fallen again. | 31.8 | 171 |
| c35 | 声が聞こえないって？マイク大丈夫？ | You can't hear my voice? Is the mic okay? | Can you hear me? Is the mic okay? | 57.5 | 222 |
| c36 | 笑わないでよ！ | Don't laugh! | Don't laugh! | 100.0 | 111 |
| c37 | ホロライブの3期生です。 | I'm part of hololive's third generation. | I'm a third-year Hololive member. | 33.0 | 221 |
| c38 | 次は失敗しないから見てて。 | I won't mess up next time, so watch. | Watch this, it won't fail next time. | 40.5 | 222 |
| c39 | この敵、倒さなくてもいいよ。 | You don't have to defeat this enemy. | This enemy doesn't need to be defeated. | 42.7 | 202 |
| c40 | えっと、何の話だっけ？ | Um, what were we talking about? | Hmm, what was that about? | 36.6 | 168 |
| p1a | 昨日マリン船長とコラボしたんだけど、 | Yesterday I did a collab with Captain Marine, and... | I collaborated with Captain Marine yesterday, but... | 61.9 | 206 |
| p1b | めっちゃ楽しかった。 | It was super fun. | It was really fun collabing with Captain Marine yesterday. | 22.3 | 259 |
| p2a | このゲーム、前にもやったことあるんだよね。 | I've actually played this game before. | This game has been played before, right? | 41.9 | 203 |
| p2b | でも全然覚えてない。 | But I don't remember it at all. | But I don't remember at all. | 80.7 | 191 |
| p3a | 犯人はたぶんあの人だと思う。 | I think the culprit is probably that person. | The criminal is probably that person. | 63.1 | 169 |
| p3b | いや、違うかも。 | No, maybe not. | No, maybe not. | 100.0 | 135 |
| p4a | 宿題やった？ | Did you do your homework? | Homework done? | 35.3 | 115 |
| p4b | まだやってない。 | I haven't done it yet. | I haven't done my homework yet. | 64.1 | 188 |

mean chrF 59.6; latency ms median 188 / p95 303 / max 493; load 2325 ms (prepare 12478 ms wall)

## ja-es

| id | Japanese | reference | output | chrF | ms |
|---|---|---|---|---|---|
| c01 | ぺこらは今日ホラーゲームやらないよ。 | Pekora no va a jugar un juego de terror hoy. | Pekora no juega juegos de terror hoy. | 58.5 | 439 |
| c02 | え、待って、それ本当？ | ¿Eh? Espera, ¿eso es verdad? | ¿Eh, espera, eso es verdad? | 69.2 | 220 |
| c03 | やばいやばいやばい、死ぬ死ぬ！ | ¡No, no, no, me muero, me muero! | ¡Mierda, mierda, mierda, ¡mato a alguien! | 13.2 | 390 |
| c04 | 全然怖くないし。 | No da nada de miedo. | No es nada temido. | 28.9 | 160 |
| c05 | 怖くないわけじゃないけど、大丈夫。 | No es que no tenga miedo, pero estoy bien. | Aunque no es miedo, está bien. | 35.7 | 276 |
| c06 | まだ行かないで、ちょっと待って。 | No te vayas todavía, espera un momento. | ¡Espera un momento, no te vayas! | 64.9 | 272 |
| c07 | みんな、コメントありがとう！ | ¡Gracias por los comentarios a todos! | ¡Gracias a todos los comentarios! | 74.7 | 194 |
| c08 | 今日は新しいゲームをやっていくぺこ。 | Hoy vamos a jugar un juego nuevo, peko. | ¡Hoy vamos a jugar un nuevo juego, peko! | 72.5 | 284 |
| c09 | それはマリン船長のせいだよ。 | Eso es culpa de la capitana Marine. | Eso es culpa del Capitán Marine. | 66.6 | 230 |
| c10 | 誰がそんなこと言ったの？ | ¿Quién dijo eso? | ¿Quién dijo eso? | 100.0 | 150 |
| c11 | もう二度とやらない。 | No lo vuelvo a hacer nunca más. | Ya no lo haré de nuevo. | 18.1 | 192 |
| c12 | あれ、ドアが開かない。 | ¿Eh? La puerta no se abre. | ¿Eh, el portón no se abre? | 39.6 | 247 |
| c13 | 鍵を取ってから戻ろう。 | Tomemos la llave y después volvamos. | ¡Voy a tomar la llave primero! | 25.2 | 223 |
| c14 | 左だって言ったじゃん！ | ¡Te dije que era a la izquierda! | ¡Izquierda, dije! | 38.4 | 187 |
| c15 | ちょっと音大きすぎない？ | ¿No está el sonido un poco demasiado alto? | ¿Un poco too fuerte? | 16.6 | 170 |
| c16 | 配信始めるの遅れてごめんね。 | Perdón por empezar tarde la transmisión. | Lo siento, empecé tarde la transmisión. | 57.3 | 261 |
| c17 | お腹すいたから、ちょっと休憩する。 | Tengo hambre, así que voy a tomar un descansito. | Me siento hambriento, así que un momento, tomaré una pausa. | 34.8 | 390 |
| c18 | これ、絶対罠でしょ。 | Esto seguro que es una trampa, ¿no? | ¡Definitivamente es una trampa! | 38.2 | 208 |
| c19 | 勝った！やったー！ | ¡Gané! ¡Bien! | ¡Gané! ¡Bien hecho! | 81.8 | 205 |
| c20 | 負けたくないんだけど。 | Es que no quiero perder. | No quiero perderme. | 69.4 | 132 |
| c21 | 知らなかった、そんなの。 | No sabía eso. | No sabía eso. | 100.0 | 134 |
| c22 | それはぺこらのじゃないよ。 | Eso no es de Pekora. | Eso no es de peko. | 80.9 | 189 |
| c23 | 明日も配信するから、見に来てね。 | Mañana también voy a transmitir, así que vengan a ver. | Por favor, ven a ver la transmisión de mañana. | 34.1 | 260 |
| c24 | え、なんで？意味わかんない。 | ¿Eh, por qué? No tiene sentido. | ¿Eh, por qué? No entiendo el significado. | 57.4 | 299 |
| c25 | ここで死んだら最初からやり直しだ。 | Si muero aquí, tengo que empezar de nuevo desde el principio. | Si mueres aquí, tendrás que empezar de nuevo. | 53.3 | 300 |
| c26 | 先輩に怒られちゃった。 | Mi senpai me regañó. | Se me enojó la senpai. | 29.6 | 211 |
| c27 | 別に好きじゃないし。 | Tampoco es que me guste. | No me gusta ni eso. | 27.5 | 154 |
| c28 | お疲れぺこ！また明日！ | ¡Buen trabajo, peko! ¡Hasta mañana! | ¡Pekora, te veo mañana! | 27.4 | 229 |
| c29 | ボス強すぎるんだけど。 | Este jefe es demasiado fuerte. | El boss es demasiado fuerte. | 68.2 | 190 |
| c30 | まだ一回も勝ってない。 | Todavía no he ganado ni una vez. | Aún no he ganado ni una vez. | 73.2 | 224 |
| c31 | スパチャありがとう、助かる！ | ¡Gracias por el superchat, me ayuda mucho! | Gracias, superchat, me sirve mucho! | 54.1 | 241 |
| c32 | あ、そこ宝箱あるじゃん。 | Ah, ahí hay un cofre del tesoro. | Ah, hay una caja de recompensas allí. | 22.9 | 304 |
| c33 | 行けると思ったのに。 | Pensé que lo iba a lograr... | Pensé que podíamos ir. | 32.7 | 207 |
| c34 | 嘘でしょ、また落ちた。 | No puede ser, me caí otra vez. | ¡Mentira, volvió a caer! | 12.2 | 260 |
| c35 | 声が聞こえないって？マイク大丈夫？ | ¿Que no se escucha mi voz? ¿Está bien el micrófono? | ¿El sonido no se escucha? ¿El micrófono está bien? | 60.5 | 359 |
| c36 | 笑わないでよ！ | ¡No se rían! | ¡No ríe! | 20.6 | 152 |
| c37 | ホロライブの3期生です。 | Soy de la tercera generación de hololive. | Soy de la tercera generación de hololive. | 100.0 | 303 |
| c38 | 次は失敗しないから見てて。 | La próxima no voy a fallar, así que miren. | Mira, esta vez no fallará. | 20.8 | 207 |
| c39 | この敵、倒さなくてもいいよ。 | No hace falta derrotar a este enemigo. | No es necesario vencer a este enemigo. | 45.1 | 243 |
| c40 | えっと、何の話だっけ？ | Eh... ¿de qué estábamos hablando? | ¿De qué hablamos? | 31.0 | 178 |
| p1a | 昨日マリン船長とコラボしたんだけど、 | Ayer hice una colaboración con la capitana Marine, y... | Ayer colabore con el Capitán Marine, pero... | 35.9 | 303 |
| p1b | めっちゃ楽しかった。 | Fue súper divertido. | Fue muy divertido colaborar con el capitán Marine ayer. | 38.0 | 319 |
| p2a | このゲーム、前にもやったことあるんだよね。 | Ya jugué este juego antes, ¿sabes? | Este juego lo hicimos antes, ¿verdad? | 42.8 | 243 |
| p2b | でも全然覚えてない。 | Pero no me acuerdo para nada. | Pero no me acuerdo en absoluto. | 59.6 | 211 |
| p3a | 犯人はたぶんあの人だと思う。 | Creo que el culpable probablemente es esa persona. | Creo que el culpable es esa persona. | 67.9 | 227 |
| p3b | いや、違うかも。 | No, tal vez no. | No, podría ser alguien else. | 15.5 | 170 |
| p4a | 宿題やった？ | ¿Hiciste la tarea? | ¿Hiciste la tarea? | 100.0 | 172 |
| p4b | まだやってない。 | Todavía no la hice. | Aún no lo hice. | 27.3 | 169 |

mean chrF 48.8; latency ms median 224 / p95 390 / max 439; load 0 ms (prepare 11289 ms wall)

