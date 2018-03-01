(
// read a whole sound into memory
s = Server.local;
// note: not *that* columbia, the first one
b = Buffer.read(s, "tmp/techno.flac");
)



(
~player = { |rate=1|
	var phase = Phasor.ar(0, BufRateScale.kr(b) * rate, 0, BufFrames.kr(b));
	BufRd.ar(2, b, phase, 1, 1)
};
)

p = ~player.play;

p.free;


p.set(\rate, 1);
p.set(\rate, 1);

p.get(\rate, { |value| ("rate is now:" + value).postln; });


// TODO: Add phase introspection with ControlBus
// http://modularbrains.net/dx490a/DX490A_su2010_02.1_[Server-language_communication].html