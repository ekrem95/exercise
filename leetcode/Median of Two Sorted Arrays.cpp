#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    double findMedianSortedArrays(vector<int>& nums1, vector<int>& nums2) {
        vector<int> nums3;

        int i, j, l1, l2;
        i = j = 0;
        l1 = nums1.size();
        l2 = nums2.size();

        if (l1 == 0 && l2 == 0) {
            return 0;
        }

        while (i < l1 && j < l2) {
            if (nums1[i] < nums2[j]){
                nums3.push_back(nums1[i]);
                ++i;
            }else {
                nums3.push_back(nums2[j]);
                ++j;
            }
        }

        while (i < l1) {
            nums3.push_back(nums1[i]);
            ++i;
        }

        while (j < l2) {
            nums3.push_back(nums2[j]);
            ++j;
        }

        if (nums3.size() % 2 == 0){
            return double(nums3.at(nums3.size()/2-1) + nums3.at(nums3.size()/2))/2;
        }
        return nums3.at(nums3.size()/2);
    }
};