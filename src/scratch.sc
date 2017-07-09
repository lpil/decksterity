b = Buffer.read(s, "/home/louis/data/new-music/breakcore/Ghost - Mu Fu Da Jiang Jun/17 Ghost - Trip 2 the Moon II.flac");
b.free;

(
x = SynthDef("tutorial-PlayBuf", { |out = 0, trigger = 1, bufnum|
    var p = PlayBuf.ar(
        numChannels: 2,
        bufnum: bufnum,
        rate: 1,
        trigger: trigger
    );
    Out.ar(out, p)
}).play(s,[\bufnum, b.bufnum ]);
x.set(\trigger, 0);
)


x.set(\trigger, 1);
x.set(\trigger, 0);
x.free;
