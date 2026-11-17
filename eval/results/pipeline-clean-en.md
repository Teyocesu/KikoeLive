# Pipeline run: `eval/data/tts/clean.wav`

- corpus CER (ASR vs truth): 0.117
- mean chrF en: Some(62.5), es: None
- speech start -> subtitle (en): n=45 median 3024 / p95 5796 / max 9637
- speech end -> subtitle (en): n=45 median 1461 / p95 4086 / max 7467
- speech start -> subtitle (es): n/a
- speech end -> subtitle (es): n/a
- ASR ms: n=49 median 217 / p95 709 / max 1034
- MT ms en: n=49 median 639 / p95 1173 / max 5755 | es: n/a
- MT queue wait en: n=49 median 0 / p95 628 / max 3064 | es: n/a
- segments 49 (forced cuts 0, blips 0), filtered 0, dropped asr-queue 0 / asr-stale 0 / mt-queue 0 / mt-stale 0
- translation failures 0, worker restarts 0, coordinator CoordinatorStats { transcripts: 49, translations: 49, stale_epoch: 0, wrong_language: 0, out_of_order: 0, duplicate: 0 }
- stale-language publications after switch: 0
- switches: []
- stop: 48 ms (joined true)
- capture: CaptureStats { packets: 12572, frames: 6034268, dropped_packets: 0, discontinuities: 0, reopen_count: 0 }

| id | reference ja | ASR | CER | lang | reference | subtitle | chrF | start→sub ms | end→sub ms |
|---|---|---|---|---|---|---|---|---|---|
| c01 | ぺこらは今日ホラーゲームやらないよ。 | ペコラは今日ホラーゲームやらないよ。 | 0.18 | en | Pekora isn't playing a horror game today. | Pekora isn't playing a horror game today. | 100 | 9637 | 7467 |
| c02 | え、待って、それ本当？ | え、待って、それ本当。 | 0.00 | en | Wait, what? Is that true? | Wait, really? | 18 | 6930 | 4990 |
| c03 | やばいやばいやばい、死ぬ死ぬ！ | やばいやばいやばい、死ぬ死ぬ。 | 0.00 | en | Oh no, oh no, oh no, I'm gonna die, I'm gonna die! | Oh no, oh no, oh no, I'm gonna die. | 69 | 5796 | 4086 |
| c04 | 全然怖くないし。 | 全然怖くないし | 0.00 | en | It's not scary at all. | It's not scary at all. | 100 | 3445 | 2125 |
| c05 | 怖くないわけじゃないけど、大丈夫。 | 怖くないわけじゃないけど、大丈夫。 | 0.00 | en | It's not that I'm not scared, but I'm okay. | It's not exactly scary, but I'm okay. | 50 | 3761 | 1551 |
| c06 | まだ行かないで、ちょっと待って。 | まだ行かないで、ちょっと待って。 | 0.00 | en | Don't go yet, wait a second. | Don't go yet, wait a little. | 67 | 3024 | 1444 |
| c07 | みんな、コメントありがとう！ | みんな、コメントありがとう。 | 0.00 | en | Thanks for the comments, everyone! | Thanks for all the comments, everyone! | 89 | 3449 | 1569 |
| c08 | 今日は新しいゲームをやっていくぺこ。 | 今日は新しいゲームをやっていくペコ | 0.12 | en | Today we're going to play a new game, peko. | Today, Peko will play a new game! | 47 | 3432 | 1472 |
| c09 | それはマリン船長のせいだよ。 | それはマリン船長のせいだよ。 | 0.00 | en | That's Captain Marine's fault. | That's Captain Marine's fault. | 100 | 2793 | 1323 |
| c10 | 誰がそんなこと言ったの？ | 誰がそんなこと言ったの? | 0.00 | en | Who said that? | Who said that? | 100 | 2673 | 1183 |
| c11 | もう二度とやらない。 | もう二度とやらない。 | 0.00 | en | I'm never doing this again. | I won't do it again. | 26 | 2235 | 1325 |
| c12 | あれ、ドアが開かない。 | あれ、ドアが開かない。 | 0.00 | en | Huh, the door won't open. | Wait, the door won't open. | 83 | 2728 | 1388 |
| c13 | 鍵を取ってから戻ろう。 | 鍵を取ってから戻ろう。 | 0.00 | en | Let's get the key and then go back. | I'll go back and get the key. | 46 | 2874 | 1354 |
| c14 | 左だって言ったじゃん！ | 左だって言ったじゃん。 | 0.00 | en | I told you it was left! | I told you to go left! | 51 | 2364 | 1274 |
| c15 | ちょっと音大きすぎない？ | ちょっと音大きすぎない | 0.00 | en | Isn't the sound a bit too loud? | Wait, is the volume too loud? | 34 | 2553 | 1423 |
| c16 | 配信始めるの遅れてごめんね。 | 配信始めるの遅れてごめんね | 0.00 | en | Sorry for starting the stream late. | Sorry, I started the stream a bit late. | 49 | 3832 | 1702 |
| c17 | お腹すいたから、ちょっと休憩する。 | お腹すいたから、ちょっと休憩する。 | 0.00 | en | I'm hungry, so I'll take a short break. | My stomach's rumbling, so I'm taking a short break. | 47 | 4222 | 1902 |
| c18 | これ、絶対罠でしょ。 | これ、絶対罠でしょ。 | 0.00 | en | This is definitely a trap, right? | This has to be a trap, right? | 47 | 2861 | 1461 |
| c19 | 勝った！やったー！ | 勝った。やったー。 | 0.00 | en | I won! Yay! | I won. Got it! | 22 | 1505 | -185 |
| c20 | 負けたくないんだけど。 | 負けたくないんだけど。 | 0.00 | en | I really don't want to lose. | I don't want to lose. | 70 | 2456 | 1366 |
| c21 | 知らなかった、そんなの。 | 知らなかった、そんなの。 | 0.00 | en | I didn't know that. | Didn't know that, that stuff. | 76 | 2708 | 1298 |
| c22 | それはぺこらのじゃないよ。 | それはペコラノじゃないよ。 | 0.33 | en | That's not Pekora's. | That's not Pekora at all. | 78 | 3087 | 1577 |
| c23 | 明日も配信するから、見に来てね。 | 明日も配信するから、見に来てね。 | 0.00 | en | I'm streaming tomorrow too, so come watch. | I'll be streaming tomorrow, so come watch! | 73 | 3974 | 1704 |
| c24 | え、なんで？意味わかんない。 | え、なんで?意味わかんない | 0.00 | en | Huh, why? That makes no sense. | Wait, why? I don't get it. | 16 | 1940 | -140 |
| c25 | ここで死んだら最初からやり直しだ。 | ここで死んだら最初からやり直しだ。 | 0.00 | en | If I die here, I have to start over from the beginning. | If I die here, I have to start over from the beginning. | 100 | 4294 | 1873 |
| c26 | 先輩に怒られちゃった。 | 先輩に怒られちゃった。 | 0.00 | en | I got scolded by my senpai. | I got scolded by senpai. | 82 | 2809 | 1459 |
| c27 | 別に好きじゃないし。 | 別に好きじゃないし。 | 0.00 | en | It's not like I like it or anything. | I don't really like it. | 21 | 2340 | 1330 |
| c28 | お疲れぺこ！また明日！ | お疲れペコ。また明日。 | 0.22 | en | Good work today, peko! See you tomorrow! | Goodnight, peko. See you tomorrow. | 51 | 2176 | -224 |
| c29 | ボス強すぎるんだけど。 | ボス強すぎるんだけど | 0.00 | en | This boss is way too strong. | The boss is way too strong though. | 75 | 2712 | 1492 |
| c30 | まだ一回も勝ってない。 | まだ一回も買ってない。 | 0.10 | en | I haven't won even once yet. | I haven't bought it even once yet. | 70 | 2483 | 1423 |
| c31 | スパチャありがとう、助かる！ | スパチャありがとう。助かる。 | 0.00 | en | Thanks for the superchat, that really helps! | Thanks for the superchat. I'm so relieved. | 56 | 3769 | 1709 |
| c32 | あ、そこ宝箱あるじゃん。 | あ、そこ宝箱あるじゃん。 | 0.00 | en | Oh, there's a treasure chest right there. | Oh, there's a treasure chest right there. | 100 | 3172 | 1472 |
| c33 | 行けると思ったのに。 | いけると思ったのに。 | 0.11 | en | I thought I could make it... | I thought I could go. | 60 | 2263 | 1313 |
| c34 | 嘘でしょ、また落ちた。 | 嘘でしょ。また落ちた。 | 0.00 | en | No way, I fell again. | That's a lie. I fell again. | 55 | 3360 | 1640 |
| c35 | 声が聞こえないって？マイク大丈夫？ | 声が聞こえないって。マイク大丈夫 | 0.00 | en | You can't hear my voice? Is the mic okay? | You can't hear the voice. The mic is fine. | 51 | 2382 | -318 |
| c36 | 笑わないでよ！ | 笑わないでよ。 | 0.00 | en | Don't laugh! | Don't laugh at me. | 80 | 1942 | 1242 |
| c37 | ホロライブの3期生です。 | ホロライブの3期生です | 0.00 | en | I'm part of hololive's third generation. | I'm a third-generation hololive member. | 51 | 3000 | 1420 |
| c38 | 次は失敗しないから見てて。 | 次は失敗しないから見てて。 | 0.00 | en | I won't mess up next time, so watch. | Next time I won't fail, so watch this. | 54 | 3400 | 1790 |
| c39 | この敵、倒さなくてもいいよ。 | この敵、倒さなくてもいいよ。 | 0.00 | en | You don't have to defeat this enemy. | This enemy doesn't have to be defeated. | 60 | 3213 | 1453 |
| c40 | えっと、何の話だっけ？ | えっと、何の話だっけ? | 0.00 | en | Um, what were we talking about? | Um, what were we talking about? | 100 | 3503 | 1563 |
| p1a | 昨日マリン船長とコラボしたんだけど、 | 昨日マリン船長とコラボしたんだけど | 0.00 | en | Yesterday I did a collab with Captain Marine, and... | I collabed with Captain Marine yesterday. | 62 | 3818 | 1738 |
| p1b | めっちゃ楽しかった。 |  | 1.00 | - |  |  | 0 | - | - |
| p2a | このゲーム、前にもやったことあるんだよね。 | めっちゃ楽しかったこのゲーム前にもやったことあるんだよね | 0.47 | en | I've actually played this game before. | That game was super fun, and I've played it before. | 38 | 4704 | 1774 |
| p2b | でも全然覚えてない。 |  | 1.00 | - |  |  | 0 | - | - |
| p3a | 犯人はたぶんあの人だと思う。 | でも全然覚えてない。犯人は多分あの人だと思う。 | 0.92 | en | I think the culprit is probably that person. | But I don't remember anything at all. I think the culprit is probably that person. | 85 | 3734 | 2204 |
| p3b | いや、違うかも。 | いや、違うかも。宿題あった。 | 0.83 | en | No, maybe not. | No, maybe not. I had homework. | 78 | 4192 | 2862 |
| p4a | 宿題やった？ |  | 1.00 | - |  |  | 0 | - | - |
| p4b | まだやってない。 | まだやってない。 | 0.00 | en | I haven't done it yet. | Not done yet. | 28 | 1829 | 1189 |
