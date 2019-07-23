# Notes

26-10-2018

Original papers:

* Kensler
* Shirley

What Wojciech is doing: explained in Owen, p 13 (Bose construction)

For next meeting:

* Create power spectrum, EPS
* create power spectra for all 2D projections
* vary:
    * which pairs of dims
    * which sampler
    * produce for a few different sample counts (ex: low, medium, high)
        * ex: []

* create python script to create all of the power spectra
* hoping that all power spectra look just like 2D jittered (for 2D projections)
* for correlated multi jittered, power spectra will probably look different
* add CMJ 3D and 4D to SampleView
* calculate EPS for jittered and unjittered for OA MJ/CMJ

# To look into (after previous tasks)

Start examining other construction methods of orthogonal arrays. For example, Bush construction (uses Galois field arithmetic). This allows for a prime raised to an arbitrary power.

# Things to mention in the meeting

- is it possible to extend asymmetrical OAs to increase the range of possible `n` values we can use? (since our construction methods limit us to $p$ and $p^2$)
